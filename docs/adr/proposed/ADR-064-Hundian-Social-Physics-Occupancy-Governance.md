# ADR-064: Hundian Social Physics Occupancy Governance & Term-Order Gate

- **Status**: Proposed
- **Date**: 2026-09-03
- **Author**: Phase Mirror Formal Methods Engineering & Hundian Economics Group
- **Decider**: PIRTM Architectural Review Board

## Context

PIRTM social physics governance models participant role allocations onto degenerate role-class sets. Naive heuristics conflated mean survey reciprocity with spin $S$, allowed unconstrained multi-occupancy, and misidentified full-shell saturation as maximum multiplicity. To establish a rigorous, machine-checked substrate for role-exclusion dynamics, PIRTM requires a formal Hundian occupancy model governed by Pauli exclusion keys, degenerate term-ordering gates, and exact multiplicity $M = n_{\text{unpaired}} + 1$.

## Decision

1. **Pauli Key & Particle Label Isolation**:
   - The Pauli Key is strictly defined as $K = (\text{role\_class}, \text{slot\_id}, \text{period\_id})$.
   - `person_id` acts strictly as a particle label and is explicitly excluded from $K$. Two participants sharing $K$ constitute dual-occupancy on a single slot.

2. **Strict Gate Order Priority**:
   - Transaction proposals MUST be evaluated through a fail-closed 5-stage gate pipeline:
     $$\text{G0 (REJ\_UNKNOWN\_CLASS)} \to \text{G1 (REJ\_DUALHAT)} \to \text{G2 (REJ\_PAULI)} \to \text{G3 (REJ\_TERM\_ORDER)} \to \text{G4/G5 (OK\_*)}$$
   - **G0 (Unknown Class)**: Role class must exist in the period-0 registered set $D$.
   - **G1 (Dual Hat)**: Participant cannot occupy another key $K'$ in the same period without an explicit waiver.
   - **G2 (Pauli Capacity)**: Maximum occupancy on key $K$ is 2 ($\alpha$ spin for 1st occupant, $\beta$ spin for 2nd occupant). A 3rd occupant attempt fails with `REJ_PAULI`.
   - **G3 (Term Order / Hund's Rule)**: For degenerate set $D$, a 2nd occupant ($\beta$) on key $K$ is legal **if and only if** all empty slots $U$ in $D$ equal 0 ($U = 0$). Attempting a 2nd occupant while any slot in $D$ is empty fails with `REJ_TERM_ORDER`.

3. **Spin & Multiplicity Derivation**:
   - Unpaired slots $n_{\text{unpaired}} = \text{count of } K \in D \text{ with occupancy } 1$.
   - System spin $S = n_{\text{unpaired}} / 2$.
   - System multiplicity $M = 2S + 1 = n_{\text{unpaired}} + 1$.
   - Maximum multiplicity occurs at half-fill ($N = |D|$, $M = |D| + 1$). Closed shell ($N = 2|D|$) yields $n_{\text{unpaired}} = 0$ and $M = 1$ (singlet state).

4. **Energy Ledger**:
   - System energy is defined as $E = V_{\text{pair}} - V_{\text{nuc}}$, where $V_{\text{pair}} \ge 0$ is pairwise friction and $V_{\text{nuc}} \ge 0$ is purpose attraction. Ground state minimizes $E$ at fixed headcount $N$ and degenerate set $D$.

## Consequences

- Formally eliminates heuristic survey reciprocity in favor of exact roster multiplicity.
- Machine-checked guarantee that pairing attempts on degenerate slots fail closed until all slots are singly occupied.
- Decouples half-fill maximum multiplicity ($M = |D| + 1$) from full-fill closed shell ($M = 1$).
