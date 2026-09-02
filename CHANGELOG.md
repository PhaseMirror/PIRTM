# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned (v2.0.0 Roadmap)
- Real-time collaborative CRDT editing engine in `pirtmd` via Yrs / Loro integration (ADR-056)
- Model-checked formal TUI state machine proofs (`Foundations.ADR.TUIStateMachine` - ADR-057)
- External Ethereum EVM Poseidon2 receipt verifier smart contract and Filecoin IPFS proof storage (ADR-058)
- Multi-user quorum governance consensus (ADR-059)
- IBC cross-chain receipt attestation bridge (ADR-060)

## [1.1.0] - 2026-09-02

### Added
- **PIRTM Daemon (`pirtmd`)**: Async WebSocket IPC background service (`ws://127.0.0.1:8090`) hosting the compiler, Sentinel gate, WardMonitor, and MCP agent (`packages/PIRTM/rust/pirtm-daemon`).
- **Interactive TUI Editor (`pirtm-tui`)**: Split-pane terminal user interface powered by Ratatui with file explorer, code editor, integrated terminal, syntax highlighting, and LSP panel (`packages/PIRTM/rust/pirtm-tui`).
- **Governance Slash Commands**: Added 14 interactive slash commands (`/compile`, `/validate`, `/status`, `/ask`, `/explain`, `/proof`, `/refactor`, `/benchmark`, `/profile`, `/deploy`, `/audit`, `/simulate`, `/certify`, `/clear`, `/quit`).
- **Editor Extensions**: External editor integrations for VS Code (`editors/vscode/`) and Neovim (`editors/neovim/`).
- **Lean 4 Proof Modules (ADR-049 through ADR-056)**:
  - `Foundations.ADR.Poseidon2Soundness` (ADR-049): Poseidon2 ZK receipt flag conjunction soundness.
  - `Foundations.ADR.DistributedGovernance` (ADR-050): Multi-node Sentinel consensus quorum soundness ($passVotes \ge quorumThreshold \iff CLUSTER\_PASS$).
  - `Foundations.ADR.InstallationProtocol` (ADR-051): Machine-checked PC local environment installation protocol.
  - `Foundations.ADR.AcePetcIntegration` (ADR-052): PETC prime valuation additive homomorphism $v_p(e_1 + e_2) = v_p(e_1) + v_p(e_2)$ and ACE weighted-$\ell_1$ soft-thresholding non-expansiveness.
  - `Foundations.ADR.UmcPmroRegulator` (ADR-053): Universal Multiplicity Constant $\Lambda_m$ fail-closed halt precedence ($stressCounter \ge 3 \implies halt$) & PMRO $2\sqrt{N}$ associator defect upper bound.
  - `Foundations.ADR.PincCdtSpacetime` (ADR-054): Regge-NCG action density operator norm bound & CDT spectral dimension proxy bounds ($1.2 \le D_s(t) \le 2.0$).
  - `Foundations.ADR.PosRatContractivity` (ADR-055): Exact rational 1-norm column sum contractivity gate $\|G\|_1 < 1$ in $\mathbb{Q}$.
  - `Foundations.ADR.CollaborativeCRDT` (ADR-056): CRDT vector clock state merge convergence & contractivity $\|G\|_1 < 1$ preservation under merged edits.
- **Rust/Kani Verification Harnesses**: Formally verified Kani model checking suites in `adr_rust` (`crdt_proof.rs`, `pinc_cdt_proof.rs`, `umc_pmro_proof.rs`, `ace_petc_proof.rs`, `distributed_governance_proof.rs`, `spectral.rs`).
- **Documentation**:
  - `TUI_USER_GUIDE.md`: Comprehensive TUI user guide and slash command reference.
  - `TUTORIAL_GOVERNED_CONTRACTS.md`: Step-by-step developer tutorial for governed contract creation.
  - `RELEASE_NOTES_v1.1.0.md`: Official v1.1.0 release notes.
  - `ADR-056-Collaborative CRDT Integration.md`: Architecture blueprint for multi-user real-time editing.

### Changed
- Realigned legal entity and trade name across all legal and governance documents to **Citizen Gardens UNA d/b/a The Prime Materia Commons** (Wyoming W.S. 17-22).
- Replaced floating-point scaling membranes ($10^6$) with canonical exact rational constructor `Ensemble::from_rationals` over reduced `PosRat` in $\mathbb{Q}$ (ADR-055).
- Updated `docker-compose.yml` to orchestrate containerized `pirtmd` daemon service on port 8090.

### Fixed
- Fixed dots (`.`) handling in Lean 4 qualified theorem anchor names within `is_theorem_anchor`.
- Resolved all residual tactic goal mismatches in `DistributedGovernance.lean`, `PosRatContractivity.lean`, `UmcPmroRegulator.lean`, and `CollaborativeCRDT.lean`.
- Clean 100% green test execution across all 25 Lean ADR modules (`lake test`) and 28 Rust workspace crates (`cargo test --workspace`).

## [1.0.0-mvp] - 2026-09-01

### Added
- Initial MVP release of PIRTM/MOC
- Lean 4 Axiom-Clean core (`lean/ADR/*.lean`, `lean/PIRTM.lean`)
- Rust compiler pipeline (`pirtm-parser`, `pirtm-mlir`, `pirtm-compiler`)
- Runtime execution engine (`pirtm-engine`) with real LLVM IR path
- WardMonitor drift detection and Zeno controller (`pirtm-monitor`)
- Standard library primitives (`pirtm-stdlib`)
- MLIR lowering for control flow, structs, enums, and FFI
- JSON parser end-to-end example (`examples/json_parser.pirtm`)
- Sedona Spine CI workflow with zero-drift toolchain locking
- 13 Architecture Decision Records (ADR-018 through ADR-030)
- Defensive publication whitepaper (`docs/DEFENSIVE_PUBLICATION_GOVERNANCE_AS_COMPILATION.md`)
- Prime Materia Open Commons License v1.0

### Security
- AdmissibilityValidator rejects float literals and uncertified primes
- Grammar quarantine enforced at crate boundary
- Proof receipts SHA-256 anchored to validated ASTs
- `SIG_GOV_KILL` fail-closed tripwire implemented

---

*For older releases, see [git tags](https://github.com/PhaseMirror/PiLang/tags).*
