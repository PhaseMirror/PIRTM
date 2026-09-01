# The Goldilocks Prime Field Backend for ZK Circuit Acceleration

- **ID**: 46
- **Status**: Accepted
- **Context**: High-performance zero-knowledge circuit generation requires accelerated modulo arithmetic over p = 2^64 - 2^32 + 1.
- **Decision**: Integrate Goldilocks prime field arithmetic (pirtm-goldilocks) for fast NTT and ZK receipt generation.
- **Consequences**:
- Accelerate Poseidon2 and PLONK proof generation.
- Preserve exact rational bounds in proof verification.
- Prove contractivity preservation in Lean 4 GoldilocksSoundness.
- **Supersedes**: none
- **Links**:
- [ADR-046 Document](../docs/adr/ADR-046-The Goldilocks prime field.md)
