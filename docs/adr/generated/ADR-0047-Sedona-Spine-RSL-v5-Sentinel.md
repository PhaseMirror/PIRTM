# Sedona Spine & RSL v5 Sentinel Integration

- **ID**: 47
- **Status**: Accepted
- **Context**: Runtime execution requires dual-layer validation binding static small-gain certificates and dynamic stress bounds under fail-closed control.
- **Decision**: Implement Sentinel validate_and_seal gate in pirtm-engine to enforce static small-gain and dynamic drift limits under SIG_GOV_KILL.
- **Consequences**:
- Re-verify small gain bounds prior to execution.
- Check dynamic rho, delta, and lambda_L_product bounds continuously.
- Emit signed receipt on pass or trigger SIG_GOV_KILL on breach.
- **Supersedes**: none
- **Links**:
- [ADR-047 Document](../docs/adr/ADR-0047-Sedona Spine & RSL v5 Sentinel.md)
