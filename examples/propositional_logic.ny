axiom a1 :: (P) :: [|- impl(not(not(P)), P)]
axiom a2 :: (P, Q) :: [|- impl(P, impl(Q, P))]
axiom a3 :: (P, Q, R) :: [|- impl(impl(P, impl(Q, R)), impl(impl(P, Q), impl(P, R)))]
axiom mp :: [P, impl(P, Q) |- Q]

theorem p_impl_p :: (P) :: [|- impl(P, P)] {
    a2(P, impl(P, P))
    a3(P, impl(P, P), P)
    mp[0, 1]
    a2(P, P)
    mp[3, 2]
}


