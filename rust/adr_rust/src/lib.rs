//! # ADR Rust — Formal verification of ADR governance and Euclidean multiplicity.
//!
//! Production-grade scaffolding for Architecture Decision Records (ADRs) with
//! Kani model checking and Euclidean multiplicity primitives.
//!
//! ## Modules
//! - `core` — ADR governance primitives (status, transition, registry, history)
//! - `euclidean` — integer hierarchy, prime factorization, divisor poset, multiplicity
//! - `proof` — Kani verification harnesses for all invariants
//! - `examples` — realistic example ADRs used by integration tests

pub mod core;
pub mod euclidean;
pub mod proof;
pub mod examples;
pub mod dialectical_semantics;
pub mod prime_quantum;
pub mod prime_autoencoder;
pub mod phase_dissonance;
pub mod governance_manifold;
pub mod cognitive_economy;
pub mod echo_braid;
pub mod floer_operator;
pub mod constitution;
pub mod license;
pub mod reconciliation;
pub mod ui_integration;

pub use core::{AdrId, AdrStatus, AdrError, ArtifactLink, Adr, AdrRegistry};
pub use examples::example_adrs;
