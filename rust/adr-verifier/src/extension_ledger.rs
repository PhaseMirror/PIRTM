//! Append-Only Cryptographic Ledger for CNL Lexicon Extension Certificates
//!
//! Implements an append-only hash-chained JSONL ledger:
//! `artifacts/cnl_extension_ledger.jsonl`.
//!
//! Chain Rules:
//! - Entry 1 (Genesis): sequence = 1, prev_hash = 64 zeros (`GENESIS_PREV_HASH`).
//! - Entry k: sequence = k, prev_hash = Entry(k-1).entry_hash.
//! - Deterministic Entry Digest: SHA-256 over sequence, prev_hash, timestamp, and certificate fields.
//! - Self-Verifying: Re-derives all certificate hashes and LawfulRecursionHash invariants on verification.

use crate::cnl_bridge::LAWFUL_RECURSION_HASH;
use crate::extension_checker::{compute_certificate_hash, ExtensionCertificate};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The genesis previous-hash value: 64 hexadecimal zeros (256-bit zero digest).
pub const GENESIS_PREV_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// Canonical relative path to the extension ledger in the project tree.
pub const DEFAULT_LEDGER_RELATIVE_PATH: &str = "artifacts/cnl_extension_ledger.jsonl";

/// An immutable record in the CNL extension ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionLedgerEntry {
    pub sequence: u64,
    pub prev_hash: String,
    pub timestamp_utc: String,
    pub certificate: ExtensionCertificate,
    pub entry_hash: String,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LedgerError {
    #[error("Non-monotonic sequence: expected {expected}, got {actual} at entry {index}")]
    NonMonotonicSequence {
        expected: u64,
        actual: u64,
        index: usize,
    },

    #[error("Hash chain break at sequence {sequence}: expected prev_hash '{expected}', got '{actual}'")]
    HashChainBreak {
        sequence: u64,
        expected: String,
        actual: String,
    },

    #[error("Entry hash mismatch at sequence {sequence}: computed '{computed}', recorded '{recorded}'")]
    EntryHashMismatch {
        sequence: u64,
        computed: String,
        recorded: String,
    },

    #[error("Certificate hash mismatch at sequence {sequence}: computed '{computed}', recorded '{recorded}'")]
    CertificateHashMismatch {
        sequence: u64,
        computed: String,
        recorded: String,
    },

    #[error("LawfulRecursionHash mismatch at sequence {sequence}: expected '{expected}', found '{actual}'")]
    LawfulRecursionHashMismatch {
        sequence: u64,
        expected: String,
        actual: String,
    },

    #[error("IO error accessing ledger at {path}: {detail}")]
    IoError { path: String, detail: String },

    #[error("JSON parse error at line {line} of ledger {path}: {detail}")]
    ParseError {
        path: String,
        line: usize,
        detail: String,
    },
}

/// Computes the deterministic SHA-256 entry hash binding sequence, prev_hash, timestamp, and certificate.
pub fn compute_entry_hash(
    sequence: u64,
    prev_hash: &str,
    timestamp_utc: &str,
    cert: &ExtensionCertificate,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sequence.to_be_bytes());
    hasher.update(prev_hash.as_bytes());
    hasher.update(timestamp_utc.as_bytes());
    hasher.update(cert.certificate_hash.as_bytes());
    hasher.update(cert.token.as_bytes());
    hasher.update(cert.prime_index.to_be_bytes());
    hasher.update(cert.class.as_bytes());
    hasher.update(cert.canonical_correspondence_hash.as_bytes());
    hasher.update(cert.lawful_recursion_hash.as_bytes());
    hex::encode(hasher.finalize()).to_lowercase()
}

/// Resolves the candidate ledger path from explicit argument or standard locations.
pub fn resolve_ledger_path(explicit: Option<&Path>) -> PathBuf {
    if let Some(p) = explicit {
        return p.to_path_buf();
    }

    let candidates = [
        PathBuf::from(DEFAULT_LEDGER_RELATIVE_PATH),
        PathBuf::from("../../../../artifacts/cnl_extension_ledger.jsonl"),
        PathBuf::from("../../artifacts/cnl_extension_ledger.jsonl"),
    ];

    for candidate in &candidates {
        if candidate.is_file() {
            return candidate.clone();
        }
    }

    for candidate in &candidates {
        if let Some(parent) = candidate.parent() {
            if parent.is_dir() {
                return candidate.clone();
            }
        }
    }

    PathBuf::from(DEFAULT_LEDGER_RELATIVE_PATH)
}

/// Loads all entries from a JSONL ledger file. Returns an empty Vec if the file does not exist.
pub fn load_ledger(path: &Path) -> Result<Vec<ExtensionLedgerEntry>, LedgerError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(path).map_err(|e| LedgerError::IoError {
        path: path.display().to_string(),
        detail: e.to_string(),
    })?;

    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for (line_idx, line_res) in reader.lines().enumerate() {
        let line = line_res.map_err(|e| LedgerError::IoError {
            path: path.display().to_string(),
            detail: e.to_string(),
        })?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let entry: ExtensionLedgerEntry =
            serde_json::from_str(trimmed).map_err(|e| LedgerError::ParseError {
                path: path.display().to_string(),
                line: line_idx + 1,
                detail: e.to_string(),
            })?;
        entries.push(entry);
    }

    Ok(entries)
}

/// Verifies the complete cryptographic hash chain and internal certificate invariants of a ledger.
pub fn verify_ledger_chain(entries: &[ExtensionLedgerEntry]) -> Result<(), LedgerError> {
    let mut expected_prev = GENESIS_PREV_HASH.to_string();

    for (idx, entry) in entries.iter().enumerate() {
        let expected_seq = (idx + 1) as u64;
        if entry.sequence != expected_seq {
            return Err(LedgerError::NonMonotonicSequence {
                expected: expected_seq,
                actual: entry.sequence,
                index: idx + 1,
            });
        }

        if entry.prev_hash != expected_prev {
            return Err(LedgerError::HashChainBreak {
                sequence: entry.sequence,
                expected: expected_prev,
                actual: entry.prev_hash.clone(),
            });
        }

        let computed_hash = compute_entry_hash(
            entry.sequence,
            &entry.prev_hash,
            &entry.timestamp_utc,
            &entry.certificate,
        );
        if computed_hash != entry.entry_hash {
            return Err(LedgerError::EntryHashMismatch {
                sequence: entry.sequence,
                computed: computed_hash,
                recorded: entry.entry_hash.clone(),
            });
        }

        let expected_cert_hash = compute_certificate_hash(
            &entry.certificate.token,
            entry.certificate.prime_index,
            &entry.certificate.class,
            entry.certificate.is_expansive,
            &entry.certificate.canonical_correspondence_hash,
            &entry.certificate.lawful_recursion_hash,
            &entry.certificate.status,
        );
        if expected_cert_hash != entry.certificate.certificate_hash {
            return Err(LedgerError::CertificateHashMismatch {
                sequence: entry.sequence,
                computed: expected_cert_hash,
                recorded: entry.certificate.certificate_hash.clone(),
            });
        }

        if entry.certificate.lawful_recursion_hash != LAWFUL_RECURSION_HASH {
            return Err(LedgerError::LawfulRecursionHashMismatch {
                sequence: entry.sequence,
                expected: LAWFUL_RECURSION_HASH.to_string(),
                actual: entry.certificate.lawful_recursion_hash.clone(),
            });
        }

        expected_prev = entry.entry_hash.clone();
    }

    Ok(())
}

/// Appends a new certificate to the ledger at `path`, returning the committed entry.
pub fn append_certificate_to_ledger(
    path: &Path,
    cert: &ExtensionCertificate,
    custom_timestamp: Option<String>,
) -> Result<ExtensionLedgerEntry, LedgerError> {
    let mut entries = load_ledger(path)?;
    verify_ledger_chain(&entries)?;

    let sequence = (entries.len() + 1) as u64;
    let prev_hash = entries
        .last()
        .map(|e| e.entry_hash.clone())
        .unwrap_or_else(|| GENESIS_PREV_HASH.to_string());

    let timestamp_utc = custom_timestamp.unwrap_or_else(|| {
        chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    });

    let entry_hash = compute_entry_hash(sequence, &prev_hash, &timestamp_utc, cert);

    let new_entry = ExtensionLedgerEntry {
        sequence,
        prev_hash,
        timestamp_utc,
        certificate: cert.clone(),
        entry_hash,
    };

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| LedgerError::IoError {
                path: parent.display().to_string(),
                detail: e.to_string(),
            })?;
        }
    }

    let serialized = serde_json::to_string(&new_entry).map_err(|e| LedgerError::ParseError {
        path: path.display().to_string(),
        line: sequence as usize,
        detail: e.to_string(),
    })?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| LedgerError::IoError {
            path: path.display().to_string(),
            detail: e.to_string(),
        })?;

    writeln!(file, "{}", serialized).map_err(|e| LedgerError::IoError {
        path: path.display().to_string(),
        detail: e.to_string(),
    })?;

    entries.push(new_entry.clone());
    verify_ledger_chain(&entries)?;

    Ok(new_entry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extension_checker::{check_extension, ExtensionCheckRequest};
    struct TempLedgerFile {
        path: PathBuf,
    }

    impl TempLedgerFile {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "test_ledger_{}_{}.jsonl",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            Self { path: p }
        }
    }

    impl Drop for TempLedgerFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    fn make_test_cert(token: &str, prime: u64, class: &str) -> ExtensionCertificate {
        let req = ExtensionCheckRequest {
            token: token.to_string(),
            proposed_prime: Some(prime),
            proposed_class: Some(class.to_string()),
            proof_path: None,
            override_canon_hash: None,
        };
        check_extension(&req).expect("Valid test certificate")
    }

    #[test]
    fn test_empty_ledger_verifies() {
        let entries: Vec<ExtensionLedgerEntry> = Vec::new();
        assert!(verify_ledger_chain(&entries).is_ok());
    }

    #[test]
    fn test_append_and_verify_chain() {
        let tmp = TempLedgerFile::new();
        let ledger_path = &tmp.path;

        let cert1 = make_test_cert("deploy", 2, "verb");
        let entry1 = append_certificate_to_ledger(
            ledger_path,
            &cert1,
            Some("2026-10-15T12:00:00Z".to_string()),
        )
        .unwrap();

        assert_eq!(entry1.sequence, 1);
        assert_eq!(entry1.prev_hash, GENESIS_PREV_HASH);
        assert!(!entry1.entry_hash.is_empty());

        let cert2 = make_test_cert("scale", 3, "verb");
        let entry2 = append_certificate_to_ledger(
            ledger_path,
            &cert2,
            Some("2026-10-15T12:01:00Z".to_string()),
        )
        .unwrap();

        assert_eq!(entry2.sequence, 2);
        assert_eq!(entry2.prev_hash, entry1.entry_hash);

        let cert3 = make_test_cert("menthol", 53, "sector");
        let entry3 = append_certificate_to_ledger(
            ledger_path,
            &cert3,
            Some("2026-10-15T12:02:00Z".to_string()),
        )
        .unwrap();

        assert_eq!(entry3.sequence, 3);
        assert_eq!(entry3.prev_hash, entry2.entry_hash);

        let entries = load_ledger(ledger_path).unwrap();
        assert_eq!(entries.len(), 3);
        assert!(verify_ledger_chain(&entries).is_ok());
    }

    #[test]
    fn test_detect_sequence_gap() {
        let cert1 = make_test_cert("deploy", 2, "verb");
        let mut entry1 = ExtensionLedgerEntry {
            sequence: 2, // Should be 1
            prev_hash: GENESIS_PREV_HASH.to_string(),
            timestamp_utc: "2026-10-15T12:00:00Z".to_string(),
            certificate: cert1.clone(),
            entry_hash: String::new(),
        };
        entry1.entry_hash = compute_entry_hash(
            entry1.sequence,
            &entry1.prev_hash,
            &entry1.timestamp_utc,
            &entry1.certificate,
        );

        let entries = vec![entry1];
        let res = verify_ledger_chain(&entries);
        assert!(matches!(res, Err(LedgerError::NonMonotonicSequence { .. })));
    }

    #[test]
    fn test_detect_hash_chain_break() {
        let cert1 = make_test_cert("deploy", 2, "verb");
        let entry1_hash = compute_entry_hash(1, GENESIS_PREV_HASH, "2026-10-15T12:00:00Z", &cert1);
        let entry1 = ExtensionLedgerEntry {
            sequence: 1,
            prev_hash: GENESIS_PREV_HASH.to_string(),
            timestamp_utc: "2026-10-15T12:00:00Z".to_string(),
            certificate: cert1,
            entry_hash: entry1_hash,
        };

        let cert2 = make_test_cert("scale", 3, "verb");
        let fake_prev = "1111111111111111111111111111111111111111111111111111111111111111";
        let entry2_hash = compute_entry_hash(2, fake_prev, "2026-10-15T12:01:00Z", &cert2);
        let entry2 = ExtensionLedgerEntry {
            sequence: 2,
            prev_hash: fake_prev.to_string(),
            timestamp_utc: "2026-10-15T12:01:00Z".to_string(),
            certificate: cert2,
            entry_hash: entry2_hash,
        };

        let entries = vec![entry1, entry2];
        let res = verify_ledger_chain(&entries);
        assert!(matches!(res, Err(LedgerError::HashChainBreak { .. })));
    }

    #[test]
    fn test_detect_entry_hash_corruption() {
        let cert1 = make_test_cert("deploy", 2, "verb");
        let entry1 = ExtensionLedgerEntry {
            sequence: 1,
            prev_hash: GENESIS_PREV_HASH.to_string(),
            timestamp_utc: "2026-10-15T12:00:00Z".to_string(),
            certificate: cert1,
            entry_hash: "corrupted_hash".to_string(),
        };

        let entries = vec![entry1];
        let res = verify_ledger_chain(&entries);
        assert!(matches!(res, Err(LedgerError::EntryHashMismatch { .. })));
    }

    #[test]
    fn test_detect_tampered_certificate() {
        let mut cert1 = make_test_cert("deploy", 2, "verb");
        cert1.prime_index = 5; // Tamper with certificate data

        let entry1_hash = compute_entry_hash(1, GENESIS_PREV_HASH, "2026-10-15T12:00:00Z", &cert1);
        let entry1 = ExtensionLedgerEntry {
            sequence: 1,
            prev_hash: GENESIS_PREV_HASH.to_string(),
            timestamp_utc: "2026-10-15T12:00:00Z".to_string(),
            certificate: cert1,
            entry_hash: entry1_hash,
        };

        let entries = vec![entry1];
        let res = verify_ledger_chain(&entries);
        assert!(matches!(res, Err(LedgerError::CertificateHashMismatch { .. })));
    }
}
