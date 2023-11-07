axiom eq_refl :: (A) :: [|- eq(A, A)]
axiom eq_symm :: [eq(A, B) |- eq(B, A)]
axiom eq_tran :: [eq(P, Q), eq(Q, R) |- eq(P, R)]
axiom eq_con1 :: (C) :: [eq(A, B) |- eq(mul(C, A), mul(C, B))]
axiom eq_con2 :: (C) :: [eq(A, B) |- eq(mul(A, C), mul(B, C))]
axiom eq_con3 :: [eq(A, B) |- eq(inv(A), inv(B))]

axiom idleft :: (A) :: [|- eq(mul(e, A), A)]
axiom invleft :: (A) :: [|- eq(mul(inv(A), A), e)]
axiom assoc :: (A, B, C) :: [|- eq(mul(mul(A, B), C), mul(A, mul(B, C)))]

theorem invright :: (A) :: [|- eq(mul(A, inv(A)), e)] {
    idleft(A)
    eq_symm[0]
    eq_con2[0](inv(A))                                   |- eq(mul(A, inv(A)), mul(mul(e, A), inv(A)))
    invleft(inv(A))
    eq_symm[0]
    eq_con2[0](A)                                        |- eq(mul(e, A), mul(mul(inv(inv(A)), inv(A)), A))
    assoc(inv(inv(A)), inv(A), A)
    eq_tran[1, 0]                                        |- eq(mul(e, A), mul(inv(inv(A)), mul(inv(A), A)))
    invleft(A)
    eq_con1[0](inv(inv(A)))
    eq_tran[2, 0]                                        |- eq(mul(e, A), mul(inv(inv(A)), e))
    eq_con2[0](inv(A))
    eq_tran[9, 0]                                        |- eq(mul(A, inv(A)), mul(mul(inv(inv(A)), e), inv(A)))
    assoc(inv(inv(A)), e, inv(A))
    eq_tran[1, 0]                                        |- eq(mul(A, inv(A)), mul(inv(inv(A)), mul(e, inv(A))))
    idleft(inv(A))
    eq_con1[0](inv(inv(A)))
    eq_tran[2, 0]                                        |- eq(mul(A, inv(A)), mul(inv(inv(A)), inv(A)))
    eq_tran[0, 14]                                       |- eq(mul(A, inv(A)), e)
}

theorem idright :: (A) :: [|- eq(mul(A, e), A)] {
    invleft(A)
    eq_symm[0]
    eq_con1[0](A)                                        |- eq(mul(A, e), mul(A, mul(inv(A), A)))
    assoc(A, inv(A), A)
    eq_symm[0]
    eq_tran[2, 0]                                        |- eq(mul(A, e), mul(mul(A, inv(A)), A))
    invright(A)
    eq_con2[0](A)
    eq_tran[2, 0]                                        |- eq(mul(A, e), mul(e, A))
    idleft(A)
    eq_tran[1, 0]                                        |- eq(mul(A, e), A)
}

theorem socks_and_shoes :: (A, B) :: [|- eq(inv(mul(A, B)), mul(inv(B), inv(A)))] {
    idright(A)
    eq_symm[0]                                           |- eq(A, mul(A, e))
    invright(B)
    eq_symm[0]                                           |- eq(e, mul(B, inv(B)))
    eq_con1[0](A)
    eq_tran[3, 0]                                        |- eq(A, mul(A, mul(B, inv(B))))
    eq_con2[0](inv(A))
    invright(A)
    eq_symm[0]
    eq_tran[0,2]                                         |- eq(e, mul(mul(A, mul(B, inv(B))), inv(A)))
    assoc(A, B, inv(B))
    eq_symm[0]
    eq_con2[0](inv(A))
    eq_tran[3, 0]                                        |- eq(e, mul(mul(mul(A, B), inv(B)), inv(A)))
    assoc(mul(A, B), inv(B), inv(A))
    eq_tran[1, 0]                                        |- eq(e, mul(mul(A, B), mul(inv(B), inv(A))))
    eq_con1[0](inv(mul(A, B)))
    idright(inv(mul(A, B)))
    eq_symm[0]
    eq_tran[0, 2]                                        |- eq(inv(mul(A, B)), mul(inv(mul(A, B)), mul(mul(A, B), mul(inv(B), inv(A)))))
    assoc(inv(mul(A, B)), mul(A, B), mul(inv(B), inv(A)))
    eq_symm[0]
    eq_tran[2, 0]                                        |- eq(inv(mul(A, B)), mul(mul(inv(mul(A, B)), mul(A, B)), mul(inv(B), inv(A))))
    invleft(mul(A, B))
    eq_con2[0](mul(inv(B), inv(A)))
    eq_tran[2, 0]                                        |- eq(inv(mul(A, B)), mul(e, mul(inv(B), inv(A))))
    idleft(mul(inv(B), inv(A)))
    eq_tran[1, 0]                                        |- eq(inv(mul(A, B)), mul(inv(B), inv(A)))
}
