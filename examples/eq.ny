axiom v_a :: [|- var(a)]
axiom v_b :: [|- var(b)]
axiom v_c :: [|- var(c)]
axiom v_d :: [|- var(d)]

axiom d_a_b :: [|- distinct(a, b)]
axiom d_a_c :: [|- distinct(a, c)]
axiom d_a_d :: [|- distinct(a, d)]
axiom d_b_c :: [|- distinct(b, c)]
axiom d_b_d :: [|- distinct(b, d)]
axiom d_c_d :: [|- distinct(c, d)]
axiom d_symm :: [distinct(A, B) |- distinct(B, A)]

axiom eq_refl :: (A) :: [|- eq(A, A)]
axiom eq_symm :: [eq(A, B) |- eq(B, A)]
axiom eq_tran :: [eq(P, Q), eq(Q, R) |- eq(P, R)]
axiom eq_cong :: (F, C) :: [eq(A, B) |- eq(apply(F, C, A), apply(F, C, B))]
axiom eq_subs :: [eq(A, B), replaced(A, C, X, Y), replaced(B, D, X, Y) |- eq(C, D)]

axiom rp_base :: (Y) :: [var(X) |- replaced(X, Y, X, Y)]
axiom rp_induct :: (F) :: [replaced(A, RA, X, Y), replaced(B, RB, X, Y) |- replaced(apply(F, A, B), apply(F, RA, RB), X, Y)]