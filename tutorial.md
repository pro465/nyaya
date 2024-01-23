# Installation
Firstly, you would need Rust and Cargo installed.
Then, just type:
```shell
cargo install --git https://github.com/pro465/nyaya
```
to install the latest version of nyaya.
then, run `nyaya` in your terminal. you should see the usage message. if you pass it a file name, and it is a syntactically valid file, and all the theorems follow from the axioms, then it should print `verified`.

# Axioms

Axioms are the foundation of maths and logic. Therefore, it makes sense for them to be also the foundation of any nyaya file.

You define axioms in the following way:
```
axiom <axiom name> :: [<conditions> |- <conclusion>]
```

for example: 

```
axiom captain_obvious :: [P |- P]
```

in this case, `captain_obvious` is the name of the axiom. It says that if you can prove `P` is true, then you can prove `P`.

(By the way, this is so obvious that it's the only class of theorems in nyaya that can be proved without any axioms.)

Note that `P` here is a variable. "names" beginning with capital letter are variables and match any valid expression, 
while those beginning with small letters are constants, and match only themselves.

You can have multiple conditions before the `|-`:

```
axiom modus_ponens :: [impl(P, Q), P |- Q]
```

in this case, `modus_ponens` is the name of the axiom, and it asserts that whenever you prove `impl(P, Q)` and `P`, you can prove `Q`.

`impl` here is a constant that causes the condition to be fulfilled only when we pass it an expression that is of the form `impl(P, Q)`, 
where `P` and `Q` are any syntactcally valid expression.

note that even though `impl` here looks like a function, it does not have to be. It could easily represent a property, a relation or even just
syntactical information (e.g., `free_var(x, S)` could represent the information that `x` is a free "variable" in `S`.)


# Valid Expressions & Pattern Matching

In the above section, we said that `P` could match any "valid expression". You are probably wondering what constitues a valid expression.

Basically, a valid expression could be a constant (`e`), a variable (`P`), or a 
"function"/"property" of other valid expressions (`f(e)`, `impl(P, Q)`, `impl(p, not(R))`, etc.)

That was easy wasn't it? Now for pattern matching:

Consider the axiom
```
axiom a :: [f(X) |- g(X)]
```

Suppose we passed it `f(h(P))`. Then, the nyaya checker would first 
check that `f(h(P))` really matches the `f(X)` in the condition of 
the axiom for some value of `X`.

It would find that it really does match the condition with `X=h(P)`.

Then, it derives `g(X)` with `X=h(P)`, thus resulting in `g(h(P))`. 
we have proved `g(h(P))` from `f(h(P))`.

On the other hand, the nyaya checker would error out if we passed to
`a` the expression `impl(h(P))` because it cannot match `f(X)` for any value of `X`.
Passing `f()` or `f(d, J)` also don't work because the axiom 
expects a "function"/"property" with arity of exactly 1.

for a more complex example, let's take the axiom `modus_ponens` defined earlier.
If we pass it two expressions: `impl(impl(P, P), P)` and `impl(P, P)`, 
the checker works out that they actually match `impl(P, Q)` and `P` respectively
with `P=impl(P, P)` and `Q=P`. It's important to know that the `P` being assigned is the axiom's 
and not the same as the `P`s appearing in the RHS, which are there only because we passed an 
expression with `P` in it.

Let's pass `modus_ponens` a pair of expressions that we intuitively know it should not accept:
`impl(not(X), Y)` and `X`. If it accepts it and gives us `not(Y)`, then it has committed a logical 
fallacy known as "denying the antecendent". But will it?

the nyaya checker first checks the first condition and finds that it indeed matches, with `P=not(X)`
and `Q=Y`. However, when it checks the second condition, it sees that the only way it could be fulfilled 
is if `P=X`, but that is not consistent with what it previously derived (i.e., `P=not(X)`). 
Hence, it rejects our proof.

# Theorems

TODO

# Arguments

TODO

# Explicit Intermediate Results

sometimes writing intermediate steps' results can be helpful for the reader to get a rough idea of how a 
theorem is proved. You could also think of it as a machine-checked proof sketch.

You write an EIR like this:
```
<use of axiom or theorem>  |- <result derived from the axiom or theorem>
```

For example:

```
theorem very_weird_theorem :: [impl(eq(p, q), eq(q, r)), eq(q, p) |- eq(q, r)] {
    eq_symmetry[0]            |- eq(p, q)
    modus_ponens[0, 2]        |- eq(q, r)
}
```

the nyaya checker also check if the results of the steps actually match with the EIRs if any, so the reader can rest assured that 
the EIRs are accurate and/or not lying, unlike comments.

# Comments

Comments start with `#` and continue till the end of the line.

For example: 
```
# this axiom gives me THE ULTIMATE POWER
axion POWER :: (P) :: [|- P]
```
