# Local PC Installation & Governed Developer Environment Protocol

- **ID**: 51
- **Status**: Accepted
- **Context**: Local installation and development of PIRTM require a sealed reproducible build pipeline and Lean 4 kernel verification.
- **Decision**: Implement install.sh pipeline, ~/.local/bin binary distribution, and Lean 4 installation soundness in InstallationProtocol.
- **Consequences**:
- Full local executable distribution in ~/.local/bin for pirtm, pirtmc, pirtm-mcp, and pirtm-lsp.
- Machine-checked zero-drift installation validation in Lean 4.
- Synchronize ADR-051 across Lean, Rust workspace, and registry.json.
- **Supersedes**: none
- **Links**:
- [ADR-051 Document](../docs/adr/ADR-051-Local-PC-Installation-Development-Protocol.md)
