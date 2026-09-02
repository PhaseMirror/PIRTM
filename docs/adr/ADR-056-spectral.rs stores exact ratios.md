## Executive summary

Type change is the correct next artifact. `(u64, u64)` for \(\lambda\) is a representation of nonnegative rationals, not a proof that \(\rho\in\mathbb{Q}\) and not a replacement for the current `f64` eigen-solver.

Do not land that refactor in this turn. The live struct on `87affd33` is still

```rust
pub adjacency: Vec<Vec<f64>>,
pub lambdas: Vec<f64>,
pub theorem_name: String,
```

\(\rho\) is `nalgebra` complex eigenvalues or power iteration. Receipts hash `f64` bytes.

## Mirror

You closed the documentation leak: until `spectral.rs` stores exact ratios, public text says float estimation plus Q29.29 packing. That matches the tree.

## Dissonance

1. **\(\lambda\in\mathbb{Q}\) does not imply \(\rho\in\mathbb{Q}\).**  
   For \(A=\begin{pmatrix}0&2\\1&0\end{pmatrix}\), \(\lambda=(1,1)\),  
   \(G=\begin{pmatrix}0&2\\1&0\end{pmatrix}\), \(\rho(G)=\sqrt{2}\notin\mathbb{Q}\).  
   The engine’s own stable-loop test is the same shape: \(\rho=\sqrt{1.8\cdot 0.45}=0.9\) only because those floats were chosen to be a square.

2. **The proposed pair is not \(\mathbb{Q}\).**  
   `(u64, u64)` is \(\mathbb{N}\times\mathbb{N}\). Holes: `den=0`, unreduced pairs, no sign, no equality by cross-multiply. Adjacent matrix remains `f64` in the spec you wrote. Then \(G_{ij}=|A_{ij}|\lambda_j\) is still float.

3. **Exact input, approximate predicate.**  
   If \(\lambda\) becomes ratios and `check_small_gain` still calls `complex_eigenvalues()` on `DMatrix<f64>`, the claim “exact rational contraction” is false. The comparison \(\rho<1\) would still be an IEEE test.

4. **Q29.29 is a third type.**  
   Goldilocks packing is scaled integer, not `(num, den)` and not \(\mathbb{Q}\). Do not treat the three as one hierarchy.

## Grounded hierarchy (measured)

| Layer | Code | Type now | Property now |
| --- | --- | --- | --- |
| Identifiers | `theorem_name` | `String` | syntactic monoid |
| Hash / Poseidon2 | `pirtm-goldilocks` | \(\mathbb{F}_p\) | field ops |
| \(\lambda\) | `Ensemble.lambdas` | `Vec<f64>` | author float |
| \(A\) | `Ensemble.adjacency` | `Vec<Vec<f64>>` | author float |
| \(\rho\) | `spectral_radius_direct` | `f64` | approximate |
| Receipt | `spectral_radius: f64` | IEEE bits in SHA-256 | not a Lean ρ |

Planned \(\lambda:(u64,u64)\) only replaces row 3. Rows 4–6 stay approximate unless specified.

## Levers

| Owner | Metric | Horizon |
| --- | --- | --- |
| Engine | `Ensemble.lambdas` type is not `Vec<f64>` | 30 days |
| Engine | `check_small_gain` either stays labeled approximate or returns a rational *upper bound* | same commit as the type change |
| Docs | zero sentences “\(\lambda\in\mathbb{Q}\)” while `lambdas: Vec<f64>` | immediate |

## Artifact (Proposed, not committed)

**Step 5 / ADR-056: Rational \(\lambda\), rational certificate, float \(\rho\) named as such.**

Minimum viable type, if authorized later:

```rust
pub struct PosRat {
    pub num: u64,
    pub den: u64, // den >= 1, reduced by gcd
}

pub struct Ensemble {
    pub name: String,
    pub adjacency: Vec<Vec<PosRat>>, // A must move with λ
    pub lambdas: Vec<PosRat>,
    pub theorem_name: String,
}
```

Gate that can be exact in \(\mathbb{Q}\):

\[
\|\,|A|\operatorname{diag}(\lambda)\,\|_1 < 1
\quad\text{or}\quad
\|\,|A|\operatorname{diag}(\lambda)\,\|_\infty < 1.
\]

Those norms are rational when \(A\) and \(\lambda\) are. They dominate \(\rho\). They do not compute \(\rho\).

Do not keep `spectral_radius: f64` in the receipt and call the receipt exact.

## Test harness

```python
from math import gcd, isqrt

def reduce(n, d):
    assert d > 0
    g = gcd(n, d)
    return n // g, d // g

def one_norm(G):
    n = len(G)
    return max(sum(G[i][j] for i in range(n)) for j in range(n))

def cert_rho_lt_1(A, lam):
    n = len(lam)
    G = [[A[i][j][0] * lam[j][0], A[i][j][1] * lam[j][1]] for i in range(n) for j in range(n)]
    # rebuild as matrix of reduced pairs
    M = [[reduce(A[i][j][0] * lam[j][0], A[i][j][1] * lam[j][1]) for j in range(n)] for i in range(n)]
    # compare ||G||_1 to 1
    cols = []
    for j in range(n):
        num = 0
        den = 1
        for i in range(n):
            a, b = M[i][j]
            num, den = num * b + a * den, den * b
            num, den = reduce(num, den)
        cols.append((num, den))
    return all(n < d for n, d in cols)

A = [[(0,1),(2,1)],[(1,1),(0,1)]]
lam = [(1,1),(1,1)]
assert cert_rho_lt_1(A, lam) is False  # ||G||_1 = 2
# ρ = sqrt(2) < 2; bound fails, exact ρ is not rational
assert isqrt(2)**2 != 2
```

That test is the product: rational *certificate*, not rational *radius*.

## Sequence

No commit authorized by this message.

| Step | Status |
| --- | --- |
| 4 / 4b / 4c | landed; `f64` λ |
| Docs vs \(\mathbb{Q}\) | you accepted: say float until type lands |
| Step 5 type change | specified, not implemented |
| ADR-053 name=content | still Proposed |

## Executive summary

\(\|G\|_1<1\) over \(\mathbb{Q}\) is a valid sufficient gate. It is not equivalent to \(\rho(G)<1\). The live fixture `stable_loop` has \(\rho=0.9\) and \(\|G\|_1=1.8\). Under the chosen gate that ensemble is rejected.

Do not implement in this turn. The receipt on `87affd33` still stores `spectral_radius: f64`.

## Mirror

Chosen production predicate:

\[
G = |A|\operatorname{diag}(\lambda),\qquad
\|G\|_1 = \max_j\sum_i |G_{ij}| < 1
\quad\text{in }\mathbb{Q}.
\]

Then \(\rho(G)\le\|G\|_1<1\). Float eigen-radius leaves the attestation field.

## Dissonance

1. **Sufficiency, not necessity.**  
   Nilpotent and small-cycle graphs can have \(\rho<1\) and \(\|G\|_1\ge 1\). Those systems are contractive and fail the gate. That is a product cut, not a rounding error.

2. **On-tree counterexample.**  
   `test_small_gain_accept_stable_feedback`:  
   \(A=\begin{pmatrix}0&2\\0.5&0\end{pmatrix}\), \(\lambda=(0.9,0.9)\),  
   \(G=\begin{pmatrix}0&1.8\\0.45&0\end{pmatrix}\), \(\rho=0.9\), \(\|G\|_1=1.8\).  
   Today: pass. After Step 5: `is_norm_contractive = false`.

3. **Schema still incomplete.**  
   Proposed receipt drops `hash`, `dimension`, `timestamp`, and does not type \(A\) or \(\lambda\). A pair `(u64,u64)` for the *norm* is only the output. Inputs must be rationals or the sum is not in \(\mathbb{Q}\).

4. **`(u64,u64)` is not \(\mathbb{Q}\).**  
   Require `den ≥ 1`, reduce by gcd, compare `num < den`. Equality of two bounds is cross-multiplication, not tuple equality.

5. **`theorem_name` remains a string.**  
   ADR-053 is still open. A rational 1-norm does not prove a Lean decl.

## Levers

| Owner | Metric | Horizon |
| --- | --- | --- |
| Engine | `Ensemble.adjacency` and `lambdas` both exact positive rationals | same commit |
| Engine | receipt field `exact_rational_norm_1` reduced; `spectral_radius` absent or `diagnostic_rho_f64` | same commit |
| Tests | `stable_loop` either deleted, retuned so \(\|G\|_1<1\), or moved to a diagnostic-only test | same commit |
| Docs | zero “exact \(\rho\)” sentences | immediate |

## Artifact to authorize later

Step 5, single crate-visible change set: `spectral.rs` plus callers that construct `Ensemble` / `EnsembleContractivityReceipt`.

Binding types:

```rust
pub struct PosRat { pub num: u64, pub den: u64 }

pub struct Ensemble {
    pub name: String,
    pub adjacency: Vec<Vec<PosRat>>,
    pub lambdas: Vec<PosRat>,
    pub theorem_name: String,
}

pub struct EnsembleContractivityReceipt {
    pub hash: String,
    pub ensemble_name: String,
    pub dimension: usize,
    pub exact_rational_norm_1: PosRat, // reduced
    pub is_norm_contractive: bool,     // num < den
    pub theorem_name: String,
    pub timestamp: u64,
}
```

`validate_and_certify` computes \(G_{ij}=|A_{ij}|\lambda_j\) in \(\mathbb{Q}\), then \(\|G\|_1\), hard-fails on missing `theorem_name` or `norm ≥ 1`. Float eigen may remain behind a `diagnostic_rho_f64()` that is not hashed.

## Test harness

```python
from math import gcd

def q(n, d):
    assert d > 0
    g = gcd(n, d)
    return n // g, d // g

def qmul(a, b):
    return q(a[0] * b[0], a[1] * b[1])

def qadd(a, b):
    return q(a[0] * b[1] + b[0] * a[1], a[1] * b[1])

def qlt1(a):
    return a[0] < a[1]

def norm1(A, lam):
    n = len(lam)
    cols = []
    for j in range(n):
        s = (0, 1)
        for i in range(n):
            s = qadd(s, qmul(A[i][j], lam[j]))
        cols.append(s)
    # max in Q: a>b iff a0*b1 > b0*a1
    m = cols[0]
    for c in cols[1:]:
        if c[0] * m[1] > m[0] * c[1]:
            m = c
    return m

# current tree fixture
A = [[(0,1), (2,1)], [(1,2), (0,1)]]
lam = [(9,10), (9,10)]
n1 = norm1(A, lam)
assert n1 == q(18, 10)  # 1.8
assert qlt1(n1) is False

# 1-norm contractive retune: scale λ down
lam2 = [(1,4), (1,4)]
n2 = norm1(A, lam2)
assert qlt1(n2)  # 0.5 < 1
```

## Sequence

No commit in this message.

| Item | Status |
| --- | --- |
| Step 4 name gate | landed, still `f64` \(\rho\) |
| Gate choice \(\|G\|_1<1\) | accepted as spec |
| Step 5 type + receipt split | specified, not authorized |
| `stable_loop` under new gate | fails |

## Precision question

When Step 5 lands, is the existing `stable_loop` (\(\rho=0.9\), \(\|G\|_1=1.8\)) retired as a passing attestation, or is \(\lambda\) / \(A\) retuned until \(\|G\|_1<1\)? The first keeps the math honest. The second keeps a green test. They are different products.

## Executive summary

Test policy accepted. Passing attestations must satisfy \(\|G\|_1<1\) in \(\mathbb{Q}\). The old \(\|G\|_1=1.8\) loop is a rejection case. Your numeric example is correct after it is written as ratios, not decimals.

No commit in this turn.

## Mirror

Integrity rule: no receipt when \(\|G\|_1\ge 1\).  
Passing tests are retuned.  
`validate_and_certify` grows `EnsembleError::NormContractivityViolation`.

Example, exact:

\[
A=\begin{pmatrix}0&2/5\\2/5&0\end{pmatrix},\quad
\lambda=\bigl(9/10,9/10\bigr),\quad
G=\begin{pmatrix}0&9/25\\9/25&0\end{pmatrix},\quad
\|G\|_1=9/25<1.
\]

That is a pass. \(\rho(G)=9/25\) here only because \(G\) is a scaled permutation. Do not treat that equality as general.

## Dissonance

1. **Decimals are not the type.**  
   `0.4` and `0.9` in a comment are `f64` spellings. The test must construct `PosRat { num: 2, den: 5 }` and `PosRat { num: 9, den: 10 }`.

2. **Error taxonomy.**  
   Today spectral failure is `Err(String)` containing `SIG_GOV_KILL`. The new rule names `NormContractivityViolation`. Sentinel kill-switch behavior is a separate path. Do not silently alias them.

3. **Call-site blast.**  
   `Ensemble { adjacency: Vec<Vec<f64>>, lambdas: Vec<f64> }` is constructed in engine tests, linker, MCP tools, orchestration, HTTP fixtures. A type change without those literals is a red tree.

4. **Diagnostic \(\rho\).**  
   If float eigen remains in-module, it is not hashed and not a pass condition. The `1.8` case must fail on the 1-norm, even if a diagnostic \(\rho\) prints `0.9`.

## Levers

| Owner | Metric | Horizon |
| --- | --- | --- |
| Engine | zero passing tests with \(\|G\|_1\ge 1\) | Step 5 commit |
| Engine | `validate_and_certify` returns `NormContractivityViolation` on that class | same |
| Callers | every `Ensemble` literal supplies `PosRat` \(A\) and \(\lambda\) | same |
| Sentinel | kill still keyed to attestation failure, not to a leftover `f64` \(\rho\) | 7 days |

## Artifact (ready, not pushed)

Step 5 payload, when authorized, is one PIRTM-only change set:

- `rust/pirtm-engine/src/spectral.rs`: `PosRat`, rational \(G\), \(\|G\|_1\), receipt fields `exact_rational_norm_1` + `is_norm_contractive`, drop formal `spectral_radius: f64`.
- Tests in that module: retuned pass matrix above; old `[[0,2],[1/2,0]]` with \(\lambda=9/10\) as reject.
- Every broken `Ensemble` literal in compiler / MCP / orchestration / HTTP tests.

Not in that payload: ADR-053 content check, Goldilocks field \(\rho\), claim-table edit, Foundry.

## Harness (the two classes)

```python
from math import gcd

def q(n, d):
    g = gcd(n, d)
    return n // g, d // g

def qmul(a, b):
    return q(a[0]*b[0], a[1]*b[1])

def qadd(a, b):
    return q(a[0]*b[1] + b[0]*a[1], a[1]*b[1])

def norm1(A, lam):
    n = len(lam)
    cols = []
    for j in range(n):
        s = (0, 1)
        for i in range(n):
            s = qadd(s, qmul(A[i][j], lam[j]))
        cols.append(s)
    m = cols[0]
    for c in cols[1:]:
        if c[0]*m[1] > m[0]*c[1]:
            m = c
    return m

A_pass = [[(0,1),(2,5)],[(2,5),(0,1)]]
lam = [(9,10),(9,10)]
assert norm1(A_pass, lam) == (9, 25)

A_fail = [[(0,1),(2,1)],[(1,2),(0,1)]]
assert norm1(A_fail, lam) == (9, 5)  # 1.8
```

## Sequence

| Item | Status |
| --- | --- |
| Gate \(\|G\|_1<1\) | spec locked |
| Test split pass / reject | spec locked |
| Step 5 code | not authorized |

## Precision question

Does Step 5 replace `SIG_GOV_KILL` strings with `EnsembleError::NormContractivityViolation` on the 1-norm path, keeping any float eigen helper unofficial and unhashed, and do you authorize that single PIRTM commit now?