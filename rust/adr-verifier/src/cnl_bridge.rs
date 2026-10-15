//! Runtime bridge between `adr-verifier` and the Controlled Natural Language (CNL) compiler.
//!
//! Formalized per ADR-003, ADR-PM-0021, and `Foundations/CNL/Core.lean`:
//! - Maps domain-specific vocabulary tokens to Prime-Indexed Operators `Ap(p)`.
//! - Strictly prohibits $p = 1$ (`L0_08_PrimeOneForbidden`), rejecting expansive/negation tokens.
//! - Binds algebraic retraction to prime 19 (`Revoke` / `Cancel`).
//! - Validates the closed Lexical Anchor Table for canonical injectivity and primality.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

/// Strictly forbidden expansive prime ($p = 1$).
pub const FORBIDDEN_PRIME_ONE: u64 = 1;

/// Certified prime index for algebraic retraction / revocation ($p = 19$).
pub const REVOCATION_PRIME: u64 = 19;

/// The semantic classification of a token in the Lexical Anchor Table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CnlTokenClass {
    /// A governing action verb (rank 0: deploy, scale, destroy, revoke).
    Verb,
    /// A target entity sector (rank 1: web-service, database, cluster).
    Sector,
    /// A bounded state parameter (rank 2: replicas).
    Param,
    /// A non-composing preposition or article (on, with, the, to, at).
    Filler,
    /// A contextual anaphor binding to prior stratum target (it).
    Anaphor,
    /// Strictly expansive token mapped to prime 1 (all, not, never, etc.).
    Expansive,
}

impl CnlTokenClass {
    /// Canonical composition order rank. `None` for non-composing classes.
    pub fn role_rank(&self) -> Option<u32> {
        match self {
            CnlTokenClass::Verb => Some(0),
            CnlTokenClass::Sector => Some(1),
            CnlTokenClass::Param => Some(2),
            CnlTokenClass::Filler | CnlTokenClass::Anaphor | CnlTokenClass::Expansive => None,
        }
    }

    /// Whether this token class contributes an operator to the MOC word.
    pub fn composes(&self) -> bool {
        self.role_rank().is_some() || *self == CnlTokenClass::Expansive
    }
}

/// Dialogue interaction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueMode {
    /// Constructive assertion mode (depth increases).
    Assert,
    /// Algebraic retraction mode (depth neutralizes previous stratum via $Ap(19)$).
    Retract,
}

/// Single entry in the Lexical Anchor Table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexicalAnchorEntry {
    /// Normalized lowercase token string.
    pub token: String,
    /// Prime index $p \in \mathbb{P}$ (or 1 for forbidden expansive tokens).
    pub prime: u64,
    /// Semantic token class.
    pub class: CnlTokenClass,
    /// Whether this entry is an alias sharing its prime with a canonical anchor.
    pub is_alias: bool,
    /// Provenance/design notes.
    pub note: String,
}

/// Error types emitted during CNL lexical analysis and compilation.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum CnlError {
    #[error("Token '{token}' is unindexed in the closed Lexical Anchor Table")]
    UnindexedToken { token: String },

    #[error("Invariant L0_08 Violation: Token '{token}' maps to forbidden expansive prime 1")]
    PrimeOneForbidden { token: String },

    #[error("Primality check failed for token '{token}': index {prime} is not prime")]
    InvalidPrimeIndex { token: String, prime: u64 },

    #[error("Injectivity violation: Duplicate prime {prime} used for canonical tokens '{token1}' and '{token2}'")]
    DuplicateCanonicalPrime {
        prime: u64,
        token1: String,
        token2: String,
    },

    #[error("Empty CNL command input")]
    EmptyInput,

    #[error("Lexicon parse error: {0}")]
    ParseError(String),
}

/// Compiled representation of a single CNL command line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledCnlCommand {
    /// Raw token strings.
    pub tokens: Vec<String>,
    /// Mapped prime sequence (for composing tokens).
    pub prime_sequence: Vec<u64>,
    /// Dialogue mode evaluated for this stratum.
    pub mode: DialogueMode,
    /// True if command contains algebraic retraction operator ($Ap(19)$).
    pub is_retraction: bool,
}

/// The runtime Lexical Anchor Table and CNL compiler bridge.
#[derive(Debug, Clone)]
pub struct LexicalAnchorTable {
    entries: BTreeMap<String, LexicalAnchorEntry>,
}

impl Default for LexicalAnchorTable {
    fn default() -> Self {
        Self::canonical()
    }
}

impl LexicalAnchorTable {
    /// Constructs the canonical Lexical Anchor Table defined in ADR-003 and `cnl_lexicon_v0.txt`.
    pub fn canonical() -> Self {
        let mut table = Self {
            entries: BTreeMap::new(),
        };

        // Canonical Verbs
        table.insert("deploy", 2, CnlTokenClass::Verb, false, "fundamental constructive action");
        table.insert("scale", 3, CnlTokenClass::Verb, false, "linear multiplicative action");
        table.insert("destroy", 17, CnlTokenClass::Verb, false, "high-index prime, significant structural tension");
        table.insert("revoke", REVOCATION_PRIME, CnlTokenClass::Verb, false, "algebraic inversion operator");
        table.insert("cancel", REVOCATION_PRIME, CnlTokenClass::Verb, true, "alias of revoke; enters Retract mode");

        // Canonical Sectors
        table.insert("web-service", 5, CnlTokenClass::Sector, false, "target entity sector");
        table.insert("database", 7, CnlTokenClass::Sector, false, "persistent data sector");
        table.insert("cluster", 47, CnlTokenClass::Sector, false, "target entity sector (ADR-PM-0021)");

        // Canonical Parameters
        table.insert("replicas", 11, CnlTokenClass::Param, false, "state parameter modifier");

        // Non-composing Fillers
        table.insert("on", 23, CnlTokenClass::Filler, false, "non-composing preposition");
        table.insert("with", 29, CnlTokenClass::Filler, false, "non-composing preposition");
        table.insert("it", 31, CnlTokenClass::Anaphor, false, "non-composing anaphor binding prior stratum");
        table.insert("the", 37, CnlTokenClass::Filler, false, "non-composing article");
        table.insert("to", 41, CnlTokenClass::Filler, false, "non-composing preposition");
        table.insert("at", 43, CnlTokenClass::Filler, false, "non-composing preposition");

        // Strictly Expansive Tokens (Forbidden Prime 1)
        table.insert("all", FORBIDDEN_PRIME_ONE, CnlTokenClass::Expansive, false, "categorically refused (L0_08)");
        table.insert("not", FORBIDDEN_PRIME_ONE, CnlTokenClass::Expansive, false, "categorically refused (L0_08)");
        table.insert("any", FORBIDDEN_PRIME_ONE, CnlTokenClass::Expansive, false, "categorically refused (L0_08)");
        table.insert("every", FORBIDDEN_PRIME_ONE, CnlTokenClass::Expansive, false, "categorically refused (L0_08)");
        table.insert("none", FORBIDDEN_PRIME_ONE, CnlTokenClass::Expansive, false, "categorically refused (L0_08)");
        table.insert("without", FORBIDDEN_PRIME_ONE, CnlTokenClass::Expansive, false, "categorically refused (L0_08)");
        table.insert("never", FORBIDDEN_PRIME_ONE, CnlTokenClass::Expansive, false, "categorically refused (L0_08)");

        table
    }

    /// Inserts an entry into the table.
    pub fn insert(
        &mut self,
        token: &str,
        prime: u64,
        class: CnlTokenClass,
        is_alias: bool,
        note: &str,
    ) {
        self.entries.insert(
            token.to_lowercase(),
            LexicalAnchorEntry {
                token: token.to_lowercase(),
                prime,
                class,
                is_alias,
                note: note.to_string(),
            },
        );
    }

    /// Looks up a token in the table.
    pub fn lookup(&self, token: &str) -> Option<&LexicalAnchorEntry> {
        self.entries.get(&token.to_lowercase())
    }

    /// Returns the total number of registered anchors.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Verifies all structural invariants of the Lexical Anchor Table:
    /// 1. Injectivity of canonical (non-alias, non-expansive) anchors.
    /// 2. Primality of all non-expansive indices.
    /// 3. Correct anchoring of the retraction operator at prime 19.
    pub fn verify_invariants(&self) -> Result<(), CnlError> {
        let mut canonical_primes: BTreeMap<u64, &str> = BTreeMap::new();
        let mut has_retract = false;

        for entry in self.entries.values() {
            if entry.class == CnlTokenClass::Expansive {
                if entry.prime != FORBIDDEN_PRIME_ONE {
                    return Err(CnlError::ParseError(format!(
                        "Expansive token '{}' must map to prime 1, found {}",
                        entry.token, entry.prime
                    )));
                }
                continue;
            }

            // Verify genuine primality for all non-expansive entries
            if !is_prime(entry.prime) {
                return Err(CnlError::InvalidPrimeIndex {
                    token: entry.token.clone(),
                    prime: entry.prime,
                });
            }

            // Verify retraction anchoring
            if entry.prime == REVOCATION_PRIME {
                has_retract = true;
            }

            // Canonical injectivity
            if !entry.is_alias {
                if let Some(&existing_token) = canonical_primes.get(&entry.prime) {
                    return Err(CnlError::DuplicateCanonicalPrime {
                        prime: entry.prime,
                        token1: existing_token.to_string(),
                        token2: entry.token.clone(),
                    });
                }
                canonical_primes.insert(entry.prime, &entry.token);
            }
        }

        if !has_retract {
            return Err(CnlError::ParseError(
                "Lexical Anchor Table missing required revocation operator at prime 19".to_string(),
            ));
        }

        Ok(())
    }

    /// Compiles a command string into prime operators, enforcing invariant L0_08 and retraction.
    pub fn compile(&self, input: &str) -> Result<CompiledCnlCommand, CnlError> {
        let tokens: Vec<String> = input
            .split_whitespace()
            .map(|t| t.to_lowercase())
            .collect();

        if tokens.is_empty() {
            return Err(CnlError::EmptyInput);
        }

        let mut prime_sequence = Vec::new();
        let mut is_retraction = false;

        for token in &tokens {
            let entry = self
                .lookup(token)
                .ok_or_else(|| CnlError::UnindexedToken {
                    token: token.clone(),
                })?;

            // Gate: Invariant L0_08 (Prime 1 Strictly Forbidden)
            if entry.prime == FORBIDDEN_PRIME_ONE || entry.class == CnlTokenClass::Expansive {
                return Err(CnlError::PrimeOneForbidden {
                    token: token.clone(),
                });
            }

            // Detect Retraction ($Ap(19)$)
            if entry.prime == REVOCATION_PRIME {
                is_retraction = true;
            }

            if entry.class.composes() {
                prime_sequence.push(entry.prime);
            }
        }

        let mode = if is_retraction {
            DialogueMode::Retract
        } else {
            DialogueMode::Assert
        };

        Ok(CompiledCnlCommand {
            tokens,
            prime_sequence,
            mode,
            is_retraction,
        })
    }
}

/// Helper function verifying primality deterministically.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut d = 5u64;
    while d.saturating_mul(d) <= n {
        if n % d == 0 || n % (d + 2) == 0 {
            return false;
        }
        d = match d.checked_add(6) {
            Some(v) => v,
            None => break,
        };
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_table_invariants() {
        let table = LexicalAnchorTable::canonical();
        assert!(table.verify_invariants().is_ok());
        assert_eq!(table.len(), 22);
    }

    #[test]
    fn test_prime_one_strictly_forbidden() {
        let table = LexicalAnchorTable::canonical();
        
        let res = table.compile("deploy all cluster");
        assert!(matches!(res, Err(CnlError::PrimeOneForbidden { token }) if token == "all"));

        let res2 = table.compile("scale not database");
        assert!(matches!(res2, Err(CnlError::PrimeOneForbidden { token }) if token == "not"));
    }

    #[test]
    fn test_retraction_on_prime_19() {
        let table = LexicalAnchorTable::canonical();

        let res = table.compile("revoke web-service").expect("compile failed");
        assert_eq!(res.mode, DialogueMode::Retract);
        assert!(res.is_retraction);
        assert!(res.prime_sequence.contains(&19));

        let res_alias = table.compile("cancel database").expect("compile failed");
        assert_eq!(res_alias.mode, DialogueMode::Retract);
        assert!(res_alias.is_retraction);
        assert!(res_alias.prime_sequence.contains(&19));
    }

    #[test]
    fn test_assertion_compilation() {
        let table = LexicalAnchorTable::canonical();

        let res = table.compile("deploy web-service on cluster").expect("compile failed");
        assert_eq!(res.mode, DialogueMode::Assert);
        assert!(!res.is_retraction);
        // "on" is a non-composing filler, so prime_sequence should only include composing classes
        assert_eq!(res.prime_sequence, vec![2, 5, 47]);
    }

    #[test]
    fn test_unindexed_token_rejected() {
        let table = LexicalAnchorTable::canonical();
        let res = table.compile("deploy kubernetes cluster");
        assert!(matches!(res, Err(CnlError::UnindexedToken { token }) if token == "kubernetes"));
    }

    #[test]
    fn test_injectivity_violation_caught() {
        let mut table = LexicalAnchorTable::canonical();
        // Insert a duplicate canonical token using prime 2 (which is already deploy)
        table.insert("launch", 2, CnlTokenClass::Verb, false, "conflicting token");
        assert!(matches!(
            table.verify_invariants(),
            Err(CnlError::DuplicateCanonicalPrime { prime: 2, .. })
        ));
    }
}
