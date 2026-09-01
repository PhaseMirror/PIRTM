# Prime Recursive Foundations of Existence

- **ID**: 32
- **Status**: Accepted
- **Context**: Introduce prime‑recursive witness constructions to provide constructive existence proofs.
- **Decision**: Adopt the PrimeRecursive module as the canonical approach for encoding existential witnesses.
- **Consequences**:
- All future existence proofs must be expressed via `existsPrimeRecursive`.
- Provide library lemmas for extracting witnesses from `PrimeWitness`.
- Document the pattern in ADR‑032.
- **Supersedes**: none
- **Links**:
- [ADR-032 Document](../docs/adr/ADR-032-Prime-Recursive-Foundations-of-Existence.md)
