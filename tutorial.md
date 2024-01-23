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

TODO explain pattern matching

# Theorems

TODO

# Arguments

TODO

# Checked Intermediate Results

TODO

# Comments

Comments start with `#` and continue till the end of the line.

For example: 
```
# this axiom gives me THE ULTIMATE POWER
axion POWER :: (P) :: [|- P]
```
