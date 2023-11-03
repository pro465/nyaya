axiom abc :: [P, impl(P, Q) |- Q]
theorem def :: [not(P), impl(not(P), impl(P, P)) |- impl(P, P)] {
    abc[0, 1]        |- impl(P, P)
}
