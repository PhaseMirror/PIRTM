# Foundry Component Integration

- **ID**: 31
- **Status**: Accepted
- **Context**: PIRTM requires deterministic generation of legal artifacts. Foundry offers a proven component for template rendering.
- **Decision**: Integrate Foundry as the canonical rendering backend for all ADR‑generated documents.
- **Consequences**:
- All document pipelines must call `Foundry.render`.
- Deprecate legacy renderer in `legacy/`.
- Version‑lock Foundry to v2.3.1.
- **Supersedes**: none
- **Links**:
- [Foundry Repo](https://github.com/pirtm/foundry)
- [Commit introducing integration](git::abcd1234)
