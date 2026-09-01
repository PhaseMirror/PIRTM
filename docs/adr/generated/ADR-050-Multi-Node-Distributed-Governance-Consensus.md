# Multi-Node Distributed Governance Consensus

- **ID**: 50
- **Status**: Accepted
- **Context**: Multi-node deployments require quorum-based consensus arbitration over local Sentinel evaluation outcomes.
- **Decision**: Implement DistributedGovernanceCluster in pirtm-orchestration and prove quorum soundness in Lean 4 DistributedGovernance.
- **Consequences**:
- Enforce cluster consensus pass iff pass votes >= quorum threshold.
- Emit aggregated cluster receipts or fail-closed SIG_GOV_KILL.
- Synchronize ADR-050 across Lean, Rust, and registry.json.
- **Supersedes**: none
- **Links**:
- [ADR-050 Document](../docs/adr/ADR-050-Multi-Node-Distributed-Governance-Consensus.md)
