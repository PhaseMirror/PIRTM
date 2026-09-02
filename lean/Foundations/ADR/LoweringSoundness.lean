/-!
# MLIR Lowering Soundness & Metric Preservation

Canonical path: `lean/Foundations/ADR/LoweringSoundness.lean`.
Moved from deprecated `lean/ADR/LoweringSoundness.lean`.
-/

namespace LoweringSoundness

structure Env where
  get : Nat → Nat
  update : Nat → Nat → Env

def standardUpdate (e : Env) (addr : Nat) (val : Nat) : Env :=
  { get := fun a => if a = addr then val else e.get a,
    update := e.update }

def envDistBounded (e1 e2 : Env) : Nat → Nat
  | 0 => if e1.get 0 ≥ e2.get 0 then e1.get 0 - e2.get 0 else e2.get 0 - e1.get 0
  | k + 1 =>
    let diff := if e1.get (k + 1) ≥ e2.get (k + 1) then e1.get (k + 1) - e2.get (k + 1) else e2.get (k + 1) - e1.get (k + 1)
    envDistBounded e1 e2 k + diff

def cellDist (v1 v2 : Nat) : Nat :=
  if v1 ≥ v2 then v1 - v2 else v2 - v1

theorem cellDist_self (v : Nat) : cellDist v v = 0 := by
  dsimp [cellDist]
  split
  · exact Nat.sub_self v
  · exact Nat.sub_self v

theorem stack_alloca_distance_invariant (v1 v2 : Nat) (h : v1 = v2) :
    cellDist v1 v2 = 0 := by
  rw [h]
  exact cellDist_self v2

structure OpTransformer where
  transform : Nat → Nat
  contractive : ∀ x y, cellDist (transform x) (transform y) ≤ cellDist x y

def idOp : OpTransformer :=
  { transform := fun x => x,
    contractive := fun x y => Nat.le_refl (cellDist x y) }

def constOp (c : Nat) : OpTransformer :=
  { transform := fun _ => c,
    contractive := fun _ _ => by
      rw [cellDist_self c]
      exact Nat.zero_le _ }

theorem mlir_lowering_preserves_contractivity (op1 op2 : OpTransformer) :
    ∀ x y, cellDist (op2.transform (op1.transform x)) (op2.transform (op1.transform y)) ≤ cellDist x y := by
  intro x y
  have h1 := op2.contractive (op1.transform x) (op1.transform y)
  have h2 := op1.contractive x y
  exact Nat.le_trans h1 h2

def iterateOp (op : OpTransformer) : Nat → OpTransformer
  | 0 => idOp
  | n + 1 =>
    let prev := iterateOp op n
    { transform := fun x => op.transform (prev.transform x),
      contractive := fun x y => by
        have h1 := op.contractive (prev.transform x) (prev.transform y)
        have h2 := prev.contractive x y
        exact Nat.le_trans h1 h2 }

theorem scf_while_contractive (op : OpTransformer) (N_max : Nat) :
    ∀ x y, cellDist ((iterateOp op N_max).transform x) ((iterateOp op N_max).transform y) ≤ cellDist x y := by
  intro x y
  exact (iterateOp op N_max).contractive x y

end LoweringSoundness
