# Definition
I guess I should define the terms first:
- small letters denote variable name, capital letters denote terms (exceptions below).
- $\mathbf{V}$ denotes the set of all variable names.
- $\mathbf{F}$ denotes the set of all function names.
- $\mathbf{T}$ denotes the set of all terms.
- term definition:
    1. $x \in \mathbf{V} \Rightarrow x \in \mathbf{T}$.
    2. $f \in \mathbf{F} \land A_1, A_2, \ldots, A_n \in \mathbf{T} \Rightarrow f(A_1, A_2, \ldots, A_n) \in \mathbf{T}$.    
- $\Gamma$ and $\Delta$ denote sets $X_0, X_1, \ldots, X_n$ of assumptions and conclusions respectively.
    - note: both are effectively joined using conjunctions unlike normal sequent calculi.
- the substitution algorithm $A[Y/x]$ is defined as follows:
    1. ${\displaystyle {x[Y/x] = Y, x \in \mathbf{T}}}$.
    2. ${\displaystyle {y[Y/x] = y, x \neq y, x, y \in \mathbf{T}}}$.
    3. ${\displaystyle {f(A_1, A_2, \ldots, A_n)[Y/x] = f(A_1[Y/x], A_2[Y/x], \ldots, A_n[Y/x])}}$.
    4. ${\displaystyle {\Gamma[Y/x] = \set{a[Y/x] : a \in \Gamma}}}$.
 
# Foundations
The heart of nyaya is a single operator, $\vdash$, which is governed by the following axioms:  
    1. ${\displaystyle {{} \over \Gamma \vdash \Gamma}}$ (reflexivity)   
    2. ${\displaystyle {\Gamma \vdash \Delta, \Delta ' \over \Gamma \vdash \Delta}}$ (selection)   
    3. ${\displaystyle {\Gamma \vdash \Delta \over \Gamma[Y/x] \vdash \Delta[Y/x]}}$ (substitution)   
    4. ${\displaystyle {\Gamma \vdash \Delta \qquad \qquad \Delta \vdash \Delta ' \over \Gamma \vdash \Delta, \Delta '}}$ (transitivity)   

(where $Y \in \mathbf{T}$, $x \in \mathbf{V}$, and $\Delta, \Delta '$ denotes $\Delta \cup \Delta '$)   
That's it!

