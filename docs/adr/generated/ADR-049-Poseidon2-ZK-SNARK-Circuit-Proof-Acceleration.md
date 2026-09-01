# Poseidon2 ZK-SNARK Circuit Proof Acceleration

- **ID**: 49
- **Status**: Accepted
- **Context**: Third-party verification of governance contractivity receipts requires accelerated zero-knowledge proof generation.
- **Decision**: Integrate Poseidon2 sponge permutation circuit (5,087 constraints) over Goldilocks prime field in pirtm-goldilocks.
- **Consequences**:
- Generate 4-element field hash squeeze outputs for contractivity receipts.
- Enforce 5,087 constraint circuit bound check in Lean 4 Poseidon2Soundness.
- Embed Poseidon2 ZK receipts into GovernedHttpServer responses.
- **Supersedes**: none
- **Links**:
- [ADR-049 Document](../docs/adr/ADR-049-Poseidon2-ZK-SNARK-Circuit-Proof-Acceleration.md)
