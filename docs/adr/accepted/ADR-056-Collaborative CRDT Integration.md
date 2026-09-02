# PIRTM v2.0.0 — Roadmap & Architecture Plan

With the v1.x line complete, v2.0.0 is the natural evolution toward a **multi-user, formally verified, and externally anchored** development environment. This plan organizes the three major thrusts into a coherent development roadmap.

---

## 1. Distributed Collaborative Editing

### Vision

Transform `pirtm-tui` from a single-user editor into a **real-time collaborative development environment** where multiple developers can simultaneously edit PIRTM contracts, view each other's cursors, and receive unified governance feedback — all while maintaining the formal integrity guarantees of the Sedona Spine.

### Core Technologies

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **CRDT Engine** | [Yrs](https://docs.rs/yrs) (Rust port of Yjs) | Mature, battle-tested CRDT with rich text support |
| **Alternative** | [Loro](https://github.com/rustworks/loro-collaborative-editing) | Rust-native, WASM-ready, modern architecture |
| **Transport** | WebSocket (extend existing `pirtmd` IPC) | Reuse current daemon infrastructure |
| **Identity** | Ed25519 keys (per-developer) | Aligns with existing cryptographic attestation |

### Architecture Changes

```
┌─────────────────────────────────────────────────────────────────────┐
│                        PIRTM Collaborative Cluster                  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐           │
│   │  pirtm-tui   │    │  pirtm-tui   │    │  pirtm-tui   │           │
│   │  (Alice)    │    │  (Bob)      │    │  (Charlie)  │           │
│   └──────┬──────┘    └──────┬──────┘    └──────┬──────┘           │
│          │                  │                  │                   │
│          └──────────────────┼──────────────────┘                   │
│                             │                                      │
│                      ┌──────▼──────┐                              │
│                      │  pirtmd     │                              │
│                      │  (CRDT      │                              │
│                      │   Coordinator)│                             │
│                      └──────┬──────┘                              │
│                             │                                      │
│                      ┌──────▼──────┐                              │
│                      │  WORM       │                              │
│                      │  Ledger     │                              │
│                      └─────────────┘                              │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Feature Set

| Feature | Description | Priority |
|---------|-------------|----------|
| **Shared Document State** | Multiple users edit the same `.pirtm` file with real-time sync | P0 |
| **Remote Cursors** | See other users' cursor positions and selections | P0 |
| **Operation History** | Full undo/redo with causal ordering | P0 |
| **User Presence** | See who is online and what they're editing | P1 |
| **Governance Consensus** | `/audit` and `/certify` commands require quorum or produce aggregate receipts | P1 |
| **Conflict Resolution** | Formal verification of merge outcomes (CRDT + Lean) | P2 |

### Formal Verification Requirements

The CRDT layer must be formally verified:

1. **CRDT Correctness** — Prove that concurrent edits converge to the same document state (Lean theorem: `crdt_convergence_sound`)
2. **Governance Preservation** — Prove that the contractivity invariant `||G||₁ < 1.0` is preserved across merged edits
3. **Conflict Detection** — Prove that conflicting edits (e.g., two users deleting the same line) produce deterministic, auditable outcomes

This extends the existing proof suite with a new module: `Foundations.ADR.CollaborativeCRDT`.

---

## 2. Formal Verification of the TUI Itself

### Vision

Subject the `pirtm-tui` client and its interaction model to the same formal rigor as the compiler. This ensures that **user actions cannot violate governance invariants** — even at the UI layer.

### Current State

The TUI is written in Rust using Ratatui and Crossterm. It communicates with `pirtmd` via WebSocket JSON‑RPC. No formal verification currently covers the UI layer.

### Approach

Adopt a **model-based verification** strategy:

1. **State Machine Model** — Define the TUI's state transitions formally (in Lean)
2. **Property-Based Testing** — Use property tests to verify UI behavior against the model
3. **Model Checking** — Use TLA+ or similar to exhaustively verify the TUI's interaction model

### Verification Layers

| Layer | Method | Tool |
|-------|--------|------|
| **UI State Machine** | Formal model in Lean | `Foundations.ADR.TUIStateMachine` |
| **Command Validation** | Property tests | `proptest` + `cargo test` |
| **WebSocket Protocol** | Model checking | TLA+ or Quint |
| **End-to-End** | Trace replay | `ITFTraceReplayer` |

### Feature Set

| Feature | Description | Priority |
|---------|-------------|----------|
| **State Machine Proof** | Prove that all TUI states satisfy `||G||₁ < 1.0` after any user action | P0 |
| **Command Safety** | Prove that `/compile`, `/validate`, etc., cannot be invoked in invalid states | P0 |
| **Protocol Correctness** | Prove that WebSocket messages are well-formed and cannot cause deserialization errors | P1 |
| **UI Invariants** | Prove that the status bar accurately reflects the actual governance state | P1 |
| **Lean-TUI Integration** | Real-time Lean proof visualization in the TUI | P2 |

### Tools & References

- **RAZE-TUI** — A verification pipeline for TUI frameworks (Rust + Ada/SPARK)
- **lean-tui** — A TUI for visualizing Lean proofs
- **TLA+ model checker** in Rust — Interactive TUI for state-space exploration

---

## 3. Integration with External Governance Ledgers

### Vision

Anchoring PIRTM's governance receipts in **external, immutable ledgers** — enabling third-party verification of contractivity, spectral bounds, and audit trails without requiring the PIRTM toolchain.

### Target Ledgers

| Ledger | Purpose | Priority |
|--------|---------|----------|
| **Ethereum / EVM** | Smart contract verification of receipts | P0 |
| **Filecoin / IPFS** | Content-addressed storage of MLIR and proofs | P1 |
| **Archivum (internal)** | Existing WORM ledger (already integrated) | P0 |
| **Cosmos / IBC** | Cross-chain attestation | P2 |
| **Bitcoin (Ordinals)** | Permanent anchoring of critical receipts | P2 |

### Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     External Ledger Integration                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐   │
│   │  pirtmd     │───▶│  Receipt    │───▶│  Ethereum (EVM)     │   │
│   │  (Seal)     │    │  Aggregator │    │  Smart Contract     │   │
│   └─────────────┘    └─────────────┘    └─────────────────────┘   │
│          │                    │                    │               │
│          │                    ▼                    │               │
│          │             ┌─────────────────────┐    │               │
│          └────────────▶│  IPFS / Filecoin    │────┘               │
│                        │  (Content Storage)  │                    │
│                        └─────────────────────┘                    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Feature Set

| Feature | Description | Priority |
|---------|-------------|----------|
| **EVM Receipt Verification** | Smart contract that verifies Poseidon2 receipts on-chain | P0 |
| **IPFS Artifact Storage** | Store MLIR, Lean proofs, and audit logs on IPFS | P1 |
| **Cross-Chain Attestation** | Use IBC to propagate receipts across chains | P2 |
| **Bitcoin Ordinals** | Inscribe critical receipts into Bitcoin blocks | P2 |
| **Verification Dashboard** | Web UI to verify receipts against external ledgers | P1 |

### Formal Verification Requirements

1. **Receipt Integrity** — Prove that receipts cannot be forged or altered (Lean: `receipt_verification_sound`)
2. **Ledger Binding** — Prove that the on-chain state correctly reflects the off-chain receipt
3. **Bridge Security** — Prove that cross-chain attestations preserve the original receipt's integrity

---

## 4. Implementation Roadmap

### Phase 1: Collaborative Editing Foundation (Weeks 1-4)

| Week | Task |
|------|------|
| 1 | Integrate Yrs CRDT into `pirtmd`; implement basic shared document state |
| 2 | Extend `pirtm-tui` to support remote cursors and presence |
| 3 | Implement operation history (undo/redo with causal ordering) |
| 4 | Add formal Lean proofs for CRDT convergence |

### Phase 2: TUI Formal Verification (Weeks 5-8)

| Week | Task |
|------|------|
| 5 | Define TUI state machine in Lean (`Foundations.ADR.TUIStateMachine`) |
| 6 | Implement property tests for UI state transitions |
| 7 | Add TLA+/Quint model checking for the WebSocket protocol |
| 8 | Integrate lean-tui for real-time proof visualization |

### Phase 3: External Ledger Integration (Weeks 9-12)

| Week | Task |
|------|------|
| 9 | Implement EVM smart contract for Poseidon2 receipt verification |
| 10 | Add IPFS storage for MLIR and audit artifacts |
| 11 | Integrate cross-chain attestation (IBC) |
| 12 | Build verification dashboard and end-to-end tests |

### Phase 4: Polish & Release (Weeks 13-14)

| Week | Task |
|------|------|
| 13 | Documentation, user guides, and tutorials |
| 14 | Release v2.0.0, update ADRs (056–060), and publish announcement |

---

## 5. New ADRs Required

| ADR | Title | Description |
|-----|-------|-------------|
| **ADR-056** | Collaborative CRDT Integration | Formalizes the CRDT engine and its governance-preserving properties |
| **ADR-057** | TUI Formal Verification | Formalizes the TUI state machine and command safety properties |
| **ADR-058** | External Ledger Binding | Formalizes the integration with Ethereum, IPFS, and other external ledgers |
| **ADR-059** | Multi-User Governance Consensus | Formalizes quorum-based governance for collaborative editing |
| **ADR-060** | Cross-Chain Attestation | Formalizes the IBC bridge for cross-chain receipt propagation |

---

## 6. Success Criteria

v2.0.0 is complete when:

- [ ] Two or more users can collaboratively edit a PIRTM contract with real-time sync
- [ ] The TUI's state machine is formally verified in Lean (zero `sorry`)
- [ ] Poseidon2 receipts can be verified on Ethereum via a deployed smart contract
- [ ] All 5 new ADRs (056–060) are accepted and verified
- [ ] `cargo test --workspace` and `lake test` remain 100% green
- [ ] A public demo is available showing multi-user collaborative governance

---

## 🚀 Next Steps

Shall I proceed with:

1. **Drafting ADR-056** (Collaborative CRDT Integration) — the formal specification and Lean proof outline?
2. **Implementing the CRDT integration** — adding Yrs to `pirtmd` and extending the TUI?
3. **Building the EVM smart contract** — the on-chain receipt verifier?

I recommend starting with **ADR-056**, as it establishes the formal foundation for the collaborative editing feature. Once accepted, we can move to implementation. Let me know your preference.