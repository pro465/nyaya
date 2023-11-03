# Definition
I guess I should define the terms first:
- small letters denote variable name, capital letters denote terms (exceptions below).
- $\mathbf{V}$ denotes the set of all variable names.
- $\mathbf{F}$ denotes the set of all function names.
- term definition:
    1. if $x \in \mathbf{V}$ then $x$ is a term.
    2. if $f \in \mathbf{F}$, and $A_1, A_2, \ldots, A_n$ are terms, then so is $f(A_1, A_2, \ldots, A_n)$.    
- $\Gamma$ and $\Delta$ denote sets $X_0, X_1, \ldots, X_n$ of assumptions and conclusions respectively.
    - note: both are effectively joined using conjunctions unlike normal sequent calculi.
- the substitution algorithm $A[Y/x]$ is defined as follows:
    1. ${\displaystyle {x[Y/x] = Y}}$.
    2. ${\displaystyle {y[Y/x] = y, x \neq y}}$.
    3. ${\displaystyle {f(A_1, A_2, \ldots, A_n)[Y/x] = f(A_1[Y/x], A_2[Y/x], \ldots, A_n[Y/x])}}$.
 
# Foundations
The heart of nyaya is a single operator, $\vdash$, which is governed by the following axioms:  
    1. ${\displaystyle {{} \over \Gamma \vdash \Gamma}}$ (reflexivity)   
    2. ${\displaystyle {\Gamma \vdash \Delta, \Delta ' \over \Gamma \vdash \Delta}}$ (selection)   
    3. ${\displaystyle {\Gamma \vdash \Delta \over \Gamma[Y/x] \vdash \Delta[Y/x]}}$ (substitution)   
    4. ${\displaystyle {\Gamma \vdash \Delta \qquad \qquad \Delta \vdash \Delta ' \over \Gamma \vdash \Delta, \Delta '}}$ (transitivity)   

That's it!

