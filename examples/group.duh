axiom v_a :: [|- var(a)]
axiom v_b :: [|- var(b)]
axiom v_c :: [|- var(c)]
axiom v_d :: [|- var(d)]

axiom eq_refl :: (A) :: [|- eq(A, A)]
axiom eq_symm :: [eq(A, B) |- eq(B, A)]
axiom eq_tran :: [eq(P, Q), eq(Q, R) |- eq(P, R)]
axiom eq_con1 :: (F, C) :: [eq(A, B) |- eq(apply(F, C, A), apply(F, C, B))]
axiom eq_con2 :: (F, C) :: [eq(A, B) |- eq(apply(F, A, C), apply(F, B, C))]
axiom eq_subs :: [eq(A, B), replaced(A, C, X, Y), replaced(B, D, X, Y) |- eq(C, D)]

axiom rp_base :: (Y) :: [var(X) |- replaced(X, Y, X, Y)]
axiom rp_induct :: (F) :: [replaced(A, RA, X, Y), replaced(B, RB, X, Y) |- replaced(apply(F, A, B), apply(F, RA, RB), X, Y)]

axiom idleft :: (A) :: [|- eq(apply(mul, e, A), A)]
axiom invleft :: (A) :: [|- eq(apply(mul, inv(A), A), e)]
axiom assoc :: (A, B, C) :: [|- eq(apply(mul, apply(mul, A, B), C), apply(mul, A, apply(mul, B, C)))]

theorem invright :: (A) :: [|- eq(apply(mul, A, inv(A)), e)] {
    idleft(A)                                            |- eq(apply(mul, e, A), A)
    eq_symm[0]                                           |- eq(A, apply(mul, e, A))
    eq_con2[1](mul, inv(A))                              |- eq(apply(mul, A, inv(A)), apply(mul, apply(mul, e, A), inv(A)))
    invleft(inv(A))                                      |- eq(apply(mul, inv(inv(A)), inv(A)), e)
    eq_symm[3]                                           |- eq(e, apply(mul, inv(inv(A)), inv(A)))
    eq_con2[4](mul, A)                                   |- eq(apply(mul, e, A), apply(mul, apply(mul, inv(inv(A)), inv(A)), A))
    assoc(inv(inv(A)), inv(A), A)                        |- eq(apply(mul, apply(mul, inv(inv(A)), inv(A)), A), apply(mul, inv(inv(A)), apply(mul, inv(A), A)))
    eq_tran[5, 6]                                        |- eq(apply(mul, e, A), apply(mul, inv(inv(A)), apply(mul, inv(A), A)))
    invleft(A)                                           |- eq(apply(mul, inv(A), A), e)
    eq_con1[8](mul, inv(inv(A)))                         |- eq(apply(mul, inv(inv(A)), apply(mul, inv(A), A)), apply(mul, inv(inv(A)), e))
    eq_tran[7, 9]                                        |- eq(apply(mul, e, A), apply(mul, inv(inv(A)), e))
    eq_con2[10](mul, inv(A))                             |- eq(apply(mul, apply(mul, e, A), inv(A)), apply(mul, apply(mul, inv(inv(A)), e), inv(A)))
    eq_tran[2, 11]                                       |- eq(apply(mul, A, inv(A)), apply(mul, apply(mul, inv(inv(A)), e), inv(A)))
    assoc(inv(inv(A)), e, inv(A))                        |- eq(apply(mul, apply(mul, inv(inv(A)), e), inv(A)), apply(mul, inv(inv(A)), apply(mul, e, inv(A))))
    eq_tran[12, 13]                                      |- eq(apply(mul, A, inv(A)), apply(mul, inv(inv(A)), apply(mul, e, inv(A))))
    idleft(inv(A))                                       |- eq(apply(mul, e, inv(A)), inv(A))
    eq_con1[15](mul, inv(inv(A)))                        |- eq(apply(mul, inv(inv(A)), apply(mul, e, inv(A))), apply(mul, inv(inv(A)), inv(A)))
    eq_tran[14, 16]                                      |- eq(apply(mul, A, inv(A)), apply(mul, inv(inv(A)), inv(A)))
    eq_tran[17, 3]                                       |- eq(apply(mul, A, inv(A)), e)
}

theorem idright :: (A) :: [|- eq(apply(mul, A, e), A)] {
    invleft(A)                                           |- eq(apply(mul, inv(A), A), e)
    eq_symm[0]                                           |- eq(e, apply(mul, inv(A), A))
    eq_con1[1](mul, A)                                   |- eq(apply(mul, A, e), apply(mul, A, apply(mul, inv(A), A)))
    assoc(A, inv(A), A)                                  |- eq(apply(mul, apply(mul, A, inv(A)), A), apply(mul, A, apply(mul, inv(A), A)))
    eq_symm[3]                                           |- eq(apply(mul, A, apply(mul, inv(A), A)), apply(mul, apply(mul, A, inv(A)), A))
    eq_tran[2, 4]                                        |- eq(apply(mul, A, e), apply(mul, apply(mul, A, inv(A)), A))
    invright(A)                                          |- eq(apply(mul, A, inv(A)), e)
    eq_con2[6](mul, A)                                   |- eq(apply(mul, apply(mul, A, inv(A)), A), apply(mul, e, A))
    eq_tran[5, 7]                                        |- eq(apply(mul, A, e), apply(mul, e, A))
    idleft(A)                                            |- eq(apply(mul, e, A), A)
    eq_tran[8, 9]                                        |- eq(apply(mul, A, e), A)
}
