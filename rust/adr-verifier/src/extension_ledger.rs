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

/// An announcement of a node's ledger head state for gossip propagation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerHeadAnnouncement {
    pub sequence: u64,
    pub head_hash: String,
    pub timestamp_utc: String,
    pub entry_count: u64,
}

/// The result of attempting to synchronize a local ledger with peer entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GossipSyncResult {
    /// Peer is already synchronized with local ledger (identical head).
    InSync { sequence: u64, head_hash: String },
    /// Local ledger is ahead of peer; peer is a strict prefix.
    PeerBehind {
        local_sequence: u64,
        peer_sequence: u64,
        head_hash: String,
    },
    /// Peer provided a verified forward extension of local ledger; entries appended.
    Advanced {
        prior_sequence: u64,
        new_sequence: u64,
        prior_head_hash: String,
        new_head_hash: String,
        entries_appended: usize,
    },
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GossipSyncError {
    #[error("Divergent fork rejected: at sequence {fork_sequence}, local entry hash '{local_hash}' differs from peer entry hash '{peer_hash}' (attempted history rewrite)")]
    DivergentForkRejected {
        fork_sequence: u64,
        local_hash: String,
        peer_hash: String,
    },

    #[error("Competing fork at same height {sequence}: local head '{local_hash}' != peer head '{peer_hash}'")]
    CompetingForkAtSameHeight {
        sequence: u64,
        local_hash: String,
        peer_hash: String,
    },

    #[error("Peer chain is internally invalid: {0}")]
    InvalidPeerChain(#[from] LedgerError),

    #[error("IO error during synchronization: {0}")]
    IoError(String),
}

/// Generates a gossip head announcement from a list of verified entries.
pub fn get_ledger_head_announcement(entries: &[ExtensionLedgerEntry]) -> LedgerHeadAnnouncement {
    if let Some(last) = entries.last() {
        LedgerHeadAnnouncement {
            sequence: last.sequence,
            head_hash: last.entry_hash.clone(),
            timestamp_utc: last.timestamp_utc.clone(),
            entry_count: entries.len() as u64,
        }
    } else {
        LedgerHeadAnnouncement {
            sequence: 0,
            head_hash: GENESIS_PREV_HASH.to_string(),
            timestamp_utc: "1970-01-01T00:00:00Z".to_string(),
            entry_count: 0,
        }
    }
}

/// Evaluates gossip synchronization between a local ledger file and peer entries.
///
/// Rules:
/// 1. Self-Verifying: Peer entries must form a strictly valid cryptographic hash chain.
/// 2. Common Ancestor / Prefix Invariant:
///    - For all sequences up to min(local_len, peer_len), `local[i].entry_hash == peer[i].entry_hash`.
///    - If any entry diverges, returns `DivergentForkRejected` and aborts without modifying local ledger.
/// 3. Monotonic Progression:
///    - If `peer.len() <= local.len()`: No forward progression. Returns `InSync` or `PeerBehind`.
///    - If `peer.len() > local.len()`: Local is a verified prefix. Peer's forward entries [local_len..peer_len]
///      are verified and appended to `local_path` (unless `dry_run` is set). Returns `Advanced`.
pub fn sync_ledger_with_peer(
    local_path: &Path,
    peer_entries: &[ExtensionLedgerEntry],
    dry_run: bool,
) -> Result<GossipSyncResult, GossipSyncError> {
    // 1. Verify internal integrity of peer entries
    verify_ledger_chain(peer_entries)?;

    // 2. Load and verify local entries
    let local_entries = load_ledger(local_path).map_err(GossipSyncError::InvalidPeerChain)?;
    verify_ledger_chain(&local_entries).map_err(GossipSyncError::InvalidPeerChain)?;

    let local_len = local_entries.len();
    let peer_len = peer_entries.len();

    // Special case: local is empty, peer has valid chain
    if local_len == 0 {
        if peer_len == 0 {
            return Ok(GossipSyncResult::InSync {
                sequence: 0,
                head_hash: GENESIS_PREV_HASH.to_string(),
            });
        }
        if !dry_run {
            if let Some(parent) = local_path.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| GossipSyncError::IoError(e.to_string()))?;
                }
            }
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(local_path)
                .map_err(|e| GossipSyncError::IoError(e.to_string()))?;
            for entry in peer_entries {
                let serialized = serde_json::to_string(entry)
                    .map_err(|e| GossipSyncError::IoError(e.to_string()))?;
                writeln!(file, "{}", serialized)
                    .map_err(|e| GossipSyncError::IoError(e.to_string()))?;
            }
        }
        return Ok(GossipSyncResult::Advanced {
            prior_sequence: 0,
            new_sequence: peer_len as u64,
            prior_head_hash: GENESIS_PREV_HASH.to_string(),
            new_head_hash: peer_entries.last().unwrap().entry_hash.clone(),
            entries_appended: peer_len,
        });
    }

    // Special case: peer is empty, local has entries
    if peer_len == 0 {
        return Ok(GossipSyncResult::PeerBehind {
            local_sequence: local_len as u64,
            peer_sequence: 0,
            head_hash: local_entries.last().unwrap().entry_hash.clone(),
        });
    }

    // Check common prefix for fork detection
    let min_len = local_len.min(peer_len);
    for i in 0..min_len {
        if local_entries[i].entry_hash != peer_entries[i].entry_hash {
            if local_len == peer_len && i == min_len - 1 {
                return Err(GossipSyncError::CompetingForkAtSameHeight {
                    sequence: (i + 1) as u64,
                    local_hash: local_entries[i].entry_hash.clone(),
                    peer_hash: peer_entries[i].entry_hash.clone(),
                });
            }
            return Err(GossipSyncError::DivergentForkRejected {
                fork_sequence: (i + 1) as u64,
                local_hash: local_entries[i].entry_hash.clone(),
                peer_hash: peer_entries[i].entry_hash.clone(),
            });
        }
    }

    if peer_len < local_len {
        Ok(GossipSyncResult::PeerBehind {
            local_sequence: local_len as u64,
            peer_sequence: peer_len as u64,
            head_hash: local_entries.last().unwrap().entry_hash.clone(),
        })
    } else if peer_len == local_len {
        Ok(GossipSyncResult::InSync {
            sequence: local_len as u64,
            head_hash: local_entries.last().unwrap().entry_hash.clone(),
        })
    } else {
        // peer_len > local_len: verified forward extension
        let forward_entries = &peer_entries[local_len..peer_len];
        if !dry_run {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(local_path)
                .map_err(|e| GossipSyncError::IoError(e.to_string()))?;
            for entry in forward_entries {
                let serialized = serde_json::to_string(entry)
                    .map_err(|e| GossipSyncError::IoError(e.to_string()))?;
                writeln!(file, "{}", serialized)
                    .map_err(|e| GossipSyncError::IoError(e.to_string()))?;
            }
            // Re-verify entire file on disk
            let updated = load_ledger(local_path).map_err(GossipSyncError::InvalidPeerChain)?;
            verify_ledger_chain(&updated).map_err(GossipSyncError::InvalidPeerChain)?;
        }

        Ok(GossipSyncResult::Advanced {
            prior_sequence: local_len as u64,
            new_sequence: peer_len as u64,
            prior_head_hash: local_entries.last().unwrap().entry_hash.clone(),
            new_head_hash: peer_entries.last().unwrap().entry_hash.clone(),
            entries_appended: forward_entries.len(),
        })
    }
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

    #[test]
    fn test_gossip_sync_identical_chains_in_sync() {
        let tmp = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        append_certificate_to_ledger(&tmp.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();

        let entries = load_ledger(&tmp.path).unwrap();
        let res = sync_ledger_with_peer(&tmp.path, &entries, false).unwrap();
        assert!(matches!(res, GossipSyncResult::InSync { sequence: 1, .. }));
    }

    #[test]
    fn test_gossip_sync_peer_behind() {
        let tmp = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2 = make_test_cert("scale", 3, "verb");
        append_certificate_to_ledger(&tmp.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp.path, &cert2, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        let all_entries = load_ledger(&tmp.path).unwrap();
        let peer_prefix = vec![all_entries[0].clone()];

        let res = sync_ledger_with_peer(&tmp.path, &peer_prefix, false).unwrap();
        assert!(matches!(res, GossipSyncResult::PeerBehind { local_sequence: 2, peer_sequence: 1, .. }));
    }

    #[test]
    fn test_gossip_sync_valid_forward_extension_advances() {
        let tmp_local = TempLedgerFile::new();
        let tmp_peer = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2 = make_test_cert("scale", 3, "verb");

        // Local has cert1
        append_certificate_to_ledger(&tmp_local.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();

        // Peer has cert1 then cert2
        append_certificate_to_ledger(&tmp_peer.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_peer.path, &cert2, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        let peer_entries = load_ledger(&tmp_peer.path).unwrap();
        let res = sync_ledger_with_peer(&tmp_local.path, &peer_entries, false).unwrap();
        match res {
            GossipSyncResult::Advanced { prior_sequence, new_sequence, entries_appended, .. } => {
                assert_eq!(prior_sequence, 1);
                assert_eq!(new_sequence, 2);
                assert_eq!(entries_appended, 1);
            }
            _ => panic!("Expected Advanced result"),
        }

        let updated_local = load_ledger(&tmp_local.path).unwrap();
        assert_eq!(updated_local.len(), 2);
        assert_eq!(updated_local[1].certificate.token, "scale");
        assert!(verify_ledger_chain(&updated_local).is_ok());
    }

    #[test]
    fn test_gossip_sync_divergent_fork_rejected() {
        let tmp_local = TempLedgerFile::new();
        let tmp_peer = TempLedgerFile::new();
        let cert1 = make_test_cert("deploy", 2, "verb");
        let cert2_local = make_test_cert("scale", 3, "verb");
        let cert2_peer = make_test_cert("menthol", 53, "sector");

        append_certificate_to_ledger(&tmp_local.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_local.path, &cert2_local, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        append_certificate_to_ledger(&tmp_peer.path, &cert1, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_peer.path, &cert2_peer, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        let peer_entries = load_ledger(&tmp_peer.path).unwrap();
        let res = sync_ledger_with_peer(&tmp_local.path, &peer_entries, false);
        assert!(res.is_err(), "Competing fork at sequence 2 must be rejected");
        let err = res.unwrap_err();
        assert!(matches!(err, GossipSyncError::CompetingForkAtSameHeight { sequence: 2, .. }));

        // Ensure local ledger was NOT modified
        let local_check = load_ledger(&tmp_local.path).unwrap();
        assert_eq!(local_check.len(), 2);
        assert_eq!(local_check[1].certificate.token, "scale");
    }

    #[test]
    fn test_gossip_sync_history_reorg_fork_rejected() {
        let tmp_local = TempLedgerFile::new();
        let tmp_peer = TempLedgerFile::new();
        let cert1_a = make_test_cert("deploy", 2, "verb");
        let cert1_b = make_test_cert("scale", 3, "verb");
        let cert2 = make_test_cert("menthol", 53, "sector");

        append_certificate_to_ledger(&tmp_local.path, &cert1_a, Some("2026-10-16T00:00:00Z".to_string())).unwrap();

        append_certificate_to_ledger(&tmp_peer.path, &cert1_b, Some("2026-10-16T00:00:00Z".to_string())).unwrap();
        append_certificate_to_ledger(&tmp_peer.path, &cert2, Some("2026-10-16T00:01:00Z".to_string())).unwrap();

        let peer_entries = load_ledger(&tmp_peer.path).unwrap();
        let res = sync_ledger_with_peer(&tmp_local.path, &peer_entries, false);
        assert!(res.is_err(), "Divergent root history must be rejected");
        let err = res.unwrap_err();
        assert!(matches!(err, GossipSyncError::DivergentForkRejected { fork_sequence: 1, .. }));
    }
}
