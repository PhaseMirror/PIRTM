# Prime-Structured Tensor-Network Autoencoder (TN-AE)

- **ID**: 36
- **Status**: Accepted
- **Context**: Conventional tensor networks utilize arbitrary bond dimensions without multiplicative structure or prime-aware rank surrogates.
- **Decision**: Integrate Prime-Structured Tensor-Network Autoencoders into PIRTM's tensor representation and MLIR lowering engine.
- **Consequences**:
- Constrain bond dimensions to prime-factored integer lattices.
- Enforce differentiable rank surrogates and prime-aware regularization.
- Penalize approximate prime-exponent vector deviations.
- **Supersedes**: none
- **Links**:
- [ADR-036 Document](../docs/adr/ADR-036-Prime-Structured-Tensor-Network-Autoencoder.md)
