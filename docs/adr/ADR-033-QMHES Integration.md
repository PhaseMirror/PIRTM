## 📄 ADR-033: Quantum-Multiplicity Hybrid Encryption System (QMHES) Integration

**Status:** Accepted  
**Date:** 2026-09-01  
**Supersedes:** None  
**Verification:** Lean stability theorem port complete — `lean/ADR/QMHESStability.lean` (zero-sorry), exercised by `lake build` / `lake test` (ADR-033 line printed by TestDriver). Open debts tracked as `AX-QMHES-001`, `AX-QMHES-003`.  

---

### Context

The **Quantum-Multiplicity Hybrid Encryption System (QMHES)** (Van Gelder, April 2026) provides a comprehensive cryptographic architecture that unifies:

- **Post-quantum cryptography**: ML-KEM (FIPS 203), ML-DSA (FIPS 204), SLH-DSA (FIPS 205)
- **Quantum key distribution (QKD)**: BB84/E91 with sifting, error correction, privacy amplification
- **Multiplicity Theory feedback**: Adaptive key rotation and state adjustment via the multiplicity operator \(\mathcal{M}_t\) and coupling tensor \(\mathcal{T}_t\)
- **Byte‑exact wire protocol**: QAHES v1.0.1 with transcript binding, replay protection, and AEAD channels

The system is already partially implemented in Python (`PhaseMirror-HQ/packages/ace_zk/`) and includes a full mathematical appendix with stability proofs (Lyapunov convergence, prime‑indexed eigenmode stability, HKDF security, frequency‑map Lipschitz bounds).

**Why integrate with PIRTM/MOC?**  
- The compiler already implements a governance kernel that enforces contractivity, spectral small‑gain, and audit trails.  
- QMHES adds a concrete cryptographic use case that can be expressed in the language, verified by Lean proofs, and executed under the Small‑Gain gate.  
- The Multiplicity feedback loop aligns naturally with the WardMonitor and Zeno‑Finton drift control.  
- Formalizing QMHES in the compiler turns the cryptographic protocol into a **governed, machine‑checkable artifact**.

---

### Decision

We will integrate QMHES into the PIRTM/MOC compiler as a **cryptographic extension** of the kernel, guided by the following principles:

1. **Protocol formalization**: The QAHES v1.0.1 wire protocol will be modeled as a set of AST nodes and MLIR operations, with the same strict governance as the core language.
2. **Stability proofs in Lean**: The five main theorems from the QMHES appendix (multiplicity boundedness, Lyapunov convergence, prime eigenmode convergence, HKDF security, frequency‑map bounded sensitivity) have been ported to Lean 4 in `lean/ADR/QMHESStability.lean` (zero‑sorry) and added to the proof suite. The strong frequency‑map Lipschitz bound is a documented open refinement (`AX-QMHES-003`).
3. **FFI bindings**: The compiler will expose `extern` functions to `liboqs` (ML‑KEM, ML‑DSA, SLH‑DSA) and to a QKD simulator (or real QKD hardware via ETSI GS QKD 014).
4. **CLI subcommand**: A new `pirtm qahes` subcommand will implement the QAHES handshake and AEAD transport, with full audit logging and Small‑Gain enforcement.
5. **Adaptive feedback**: The Multiplicity feedback loop (eq. 24) will be integrated with the WardMonitor, allowing the system to adjust key rotation cadence based on observed drift and QBER.

**Non‑goals (for this ADR):**
- Replacing existing TLS/IPsec stacks; QMHES is intended as a **governed, auditable alternative** for high‑assurance applications.
- Full formal verification of the cryptographic primitives (we rely on NIST standards and `liboqs`); we verify the **protocol composition** and **stability feedback**.

---

### Consequences

#### Benefits
- **Unified security model**: The compiler now directly supports post‑quantum secure communication, with audit receipts and contractivity certificates for every cryptographic operation.
- **Machine‑checked stability**: The QMHES stability proofs become part of the Lean proof suite, ensuring that the adaptive feedback loop does not diverge.
- **Real‑world applicability**: QMHES provides a tangible use case for the compiler (secure data channels, AI‑to‑AI communication, blockchain consensus), demonstrating the language's capability beyond numerical tensor operations.
- **Interoperability**: The QAHES wire protocol is byte‑exact and can be implemented in other languages (Go, Rust, C) for cross‑platform use.

#### Costs / Risks
- **Dependency on `liboqs`**: We introduce an external C library dependency for PQC operations. This must be managed in CI and packaging.
- **Increased complexity**: Cryptographic protocols add new state machines, nonce handling, and replay protection – all of which require careful governance.
- **Performance overhead**: PQC operations (ML‑KEM, ML‑DSA) are computationally more expensive than classical equivalents; this may affect latency-sensitive applications.
- **QKD availability**: Real QKD hardware is not widely available; we will rely on a simulator for early integration and testing.

---

### Implementation Plan (Phased)

#### Phase 1: Lean Formalization (2 weeks)
- Port the five QMHES stability theorems (A.4, C.2, D.3, E.2, F.4) to Lean 4 in `lean/QMHESStability.lean`.
- Prove all theorems using the existing `BoundedIteration.lean` and `LoweringSoundness.lean` where applicable.
- Integrate `lake build --reject-sorry` to ensure zero `sorry` and zero Mathlib.

#### Phase 2: AST & Parser Extensions (1 week)
- Add new AST nodes for cryptographic primitives:
  - `Expr::KemEncapsulate { public_key: Expr }`
  - `Expr::KemDecapsulate { ciphertext: Expr, secret_key: Expr }`
  - `Expr::Sign { secret_key: Expr, message: Expr }`
  - `Expr::Verify { public_key: Expr, message: Expr, signature: Expr }`
  - `Expr::AeadEncrypt { key: Expr, nonce: Expr, plaintext: Expr, associated_data: Expr }`
  - `Expr::AeadDecrypt { key: Expr, nonce: Expr, ciphertext: Expr, associated_data: Expr }`
- Extend the type system with `KemKeyPair`, `Signature`, `AeadCiphertext`, etc.
- Update the grammar (`pirtm.pest`) to support these new constructs (optional – can be exposed via FFI initially).

#### Phase 3: FFI Bindings & Runtime (2 weeks)
- Add `liboqs` FFI functions to `pirtm-engine/src/ffi.rs`:
  - `oqs_kem_keypair(alg: *const c_char) -> KemKeyPair`
  - `oqs_kem_encapsulate(pk: *const u8) -> (ciphertext, shared_secret)`
  - `oqs_kem_decapsulate(sk: *const u8, ciphertext: *const u8) -> shared_secret`
  - `oqs_sign_keypair(alg: *const c_char) -> SignKeyPair`
  - `oqs_sign(sk: *const u8, msg: *const u8) -> signature`
  - `oqs_verify(pk: *const u8, msg: *const u8, sig: *const u8) -> bool`
- Implement the QAHES handshake state machine in Rust (or PIRTM source) with sequence validation and transcript hashing.
- Extend the `run` subcommand to accept a `--qahes` flag that sets up an encrypted channel.

#### Phase 4: Multiplicity Feedback Integration (1 week)
- Implement the `MultiplicityState` update (eq. 24) in Rust, using the WardMonitor's drift metrics as the observable state vector `s_t`.
- Wire the `rotation_needed` signal into the QAHES rekey trigger.
- Add audit logging for every key rotation event.

#### Phase 5: End‑to‑End Test & Demo (1 week)
- Write a PIRTM program that establishes a QAHES session between two peers.
- Include ML‑KEM key exchange, ML‑DSA authentication, and AEAD‑encrypted data transfer.
- Run the program under the Small‑Gain gate and verify audit receipts are generated.
- Produce a simple CLI demo (`pirtm qahes --server` and `--client`).

---

### Links

- [QMHES Defensive Publication](./docs/QMHES.pdf)
- [PhaseMirror-HQ repo](https://github.com/MultiplicityFoundation/PhaseMirror-HQ)
- [Existing ADR-013](./docs/adr/ADR-013-PIRTM-MOC-Language-Scope.md)
- [ADR-016](./docs/adr/ADR-016-Bounded-Iteration-Control-Flow.md)
- [ADR-017](./docs/adr/ADR-017-Lowering-Soundness.md)

---

### Sign‑off

**Approved by:** Governance Board (pending)  
**Date:** TBD  

*This ADR will be considered accepted once Phase 1 (Lean formalization) artifacts are merged into the tree.*