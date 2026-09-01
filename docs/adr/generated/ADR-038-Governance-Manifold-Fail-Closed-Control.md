# Phase Mirror Governance Manifold & Fail-Closed Control

- **ID**: 38
- **Status**: Accepted
- **Context**: Advisory governance allows engines to operate in unmonitored drift or corrupted states.
- **Decision**: Integrate Phase Mirror Governance Manifold as a mandatory L0 fail-closed execution substrate.
- **Consequences**:
- Couple Hamiltonian dynamics to positive semi-definite governance potential.
- Enforce discrete GovernorHalt when gain saturates and drift grows.
- Invalidate control vector caches adaptively when drift exceeds soft envelope.
- **Supersedes**: none
- **Links**:
- [ADR-038 Document](../docs/adr/ADR-038-Governance-Manifold-Fail-Closed-Control.md)
