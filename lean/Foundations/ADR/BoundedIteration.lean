/-!
# Bounded Iteration & Control-Flow Contractivity Formalization

Canonical path: `lean/Foundations/ADR/BoundedIteration.lean`.
Moved from deprecated `lean/ADR/BoundedIteration.lean`.
-/

namespace BoundedIteration

structure MetricSpace (X : Type) where
  dist : X → X → Nat
  dist_self : ∀ x, dist x x = 0
  dist_symm : ∀ x y, dist x y = dist y x
  dist_triangle : ∀ x y z, dist x z ≤ dist x y + dist y z

structure NonExpansiveMap (X : Type) (M : MetricSpace X) where
  f : X → X
  bound : ∀ x y, M.dist (f x) (f y) ≤ M.dist x y

def idMap (X : Type) : X → X := fun x => x

def iterate {X : Type} (f : X → X) : Nat → (X → X)
  | 0 => idMap X
  | n + 1 => fun x => f (iterate f n x)

theorem compose_non_expansive {X : Type} (M : MetricSpace X)
    (f g : NonExpansiveMap X M) :
    ∀ x y, M.dist (f.f (g.f x)) (f.f (g.f y)) ≤ M.dist x y := by
  intro x y
  have h1 := f.bound (g.f x) (g.f y)
  have h2 := g.bound x y
  exact Nat.le_trans h1 h2

theorem iterate_non_expansive {X : Type} (M : MetricSpace X)
    (f : NonExpansiveMap X M) (N : Nat) :
    ∀ x y, M.dist (iterate f.f N x) (iterate f.f N y) ≤ M.dist x y := by
  intro x y
  induction N with
  | zero =>
    dsimp [iterate, idMap]
    exact Nat.le_refl (M.dist x y)
  | succ n ih =>
    dsimp [iterate]
    have h1 := f.bound (iterate f.f n x) (iterate f.f n y)
    exact Nat.le_trans h1 ih

def max_bound (b1 b2 : Nat) : Nat :=
  if b1 ≥ b2 then b1 else b2

theorem conditional_branch_safe (b1 b2 radius : Nat)
    (h1 : b1 ≤ radius) (h2 : b2 ≤ radius) :
    max_bound b1 b2 ≤ radius := by
  dsimp [max_bound]
  split
  · exact h1
  · exact h2

theorem bounded_while_envelope {X : Type} (M : MetricSpace X)
    (body : NonExpansiveMap X M) (N_max : Nat) (initial_radius : Nat)
    (h_init : ∀ x y, M.dist x y ≤ initial_radius) :
    ∀ x y, M.dist (iterate body.f N_max x) (iterate body.f N_max y) ≤ initial_radius := by
  intro x y
  have h_iter := iterate_non_expansive M body N_max x y
  have h_base := h_init x y
  exact Nat.le_trans h_iter h_base

end BoundedIteration
