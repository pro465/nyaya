use crate::{
    error::{Error, ErrorTy, Loc},
    parser::{Call, Expr, Rule},
};
use std::{
    collections::{HashMap, HashSet},
    vec::IntoIter,
};

pub struct Verifier {
    rules: HashMap<String, Rule>,
    rest: IntoIter<Rule>,
}

impl Verifier {
    pub fn new(v: Vec<Rule>) -> Self {
        Self {
            rest: v.into_iter(),
            rules: HashMap::new(),
        }
    }

    pub fn verify_next(&mut self) -> Result<Option<()>, Error> {
        let a = match self.rest.next() {
            Some(x) => x,
            None => return Ok(None),
        };

        if self.rules.contains_key(&a.name) {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: "duplicate rule definition for ".to_string() + &a.name,
            });
        }
        self.check_closed(&a)?;

        if let Some(proof) = &a.proof {
            self.verify_proof(&a, proof)?;
        }

        self.rules.insert(a.name.clone(), a);

        Ok(Some(()))
    }

    fn check_closed(&self, a: &Rule) -> Result<(), Error> {
        let mut param_names = HashSet::new();
        for p in a.params.iter() {
            if !param_names.insert(p.clone()) {
                return Err(Error {
                    loc: a.loc,
                    ty: ErrorTy::VerifError,
                    desc: format!("duplicate variable declaration for {}", p),
                });
            }
        }
        let mut pat_names = HashSet::new();
        for p in a.pat.iter() {
            insert_vars(&mut pat_names, p);
        }

        if !pat_names.is_disjoint(&param_names) {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "duplicate parameter name(s): {}",
                    vars_to_string(pat_names.intersection(&param_names).cloned().collect())
                ),
            });
        }

        let declared_vars = &param_names | &pat_names;
        let rep_names = self.check_closed_expr(&a.rep, &declared_vars, a.loc)?;

        if !param_names.is_subset(&rep_names) {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "unused parameter(s): {}",
                    vars_to_string(param_names.difference(&rep_names).cloned().collect())
                ),
            });
        }

        if let Some(proof) = &a.proof {
            for call in proof {
                if !self.rules.contains_key(&call.f) {
                    return Err(Error {
                        loc: call.loc,
                        ty: ErrorTy::VerifError,
                        desc: format!("rule named `{}` not declared", call.f),
                    });
                }
                for arg in &call.args {
                    self.check_closed_expr(arg, &declared_vars, call.loc)?;
                }
                if let Some(e) = &call.ret {
                    self.check_closed_expr(e, &declared_vars, call.loc)?;
                }
            }
        }
        Ok(())
    }

    fn check_closed_expr(
        &self,
        e: &Expr,
        declared_vars: &HashSet<String>,
        loc: Loc,
    ) -> Result<HashSet<String>, Error> {
        let mut free_vars = HashSet::new();
        insert_vars(&mut free_vars, e);
        if !free_vars.is_subset(declared_vars) {
            Err(Error {
                loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "undeclared variable(s): {}",
                    vars_to_string(free_vars.difference(&declared_vars).cloned().collect())
                ),
            })
        } else {
            Ok(free_vars)
        }
    }

    fn verify_proof(&mut self, a: &Rule, proof: &[Call]) -> Result<(), Error> {
        let mut res = a.pat.clone();
        for call in proof.clone() {
            let rule = &self.rules[&call.f];
            let hyp = index(&res, &call.idxs, call.loc)?;
            let mut vars = HashMap::new();
            if rule.pat.len() != hyp.len() {
                return Err(Error {
                    loc: call.loc,
                    ty: ErrorTy::VerifError,
                    desc: format!(
                        "axiom/theorem named `{}` requires {} conditions, but invocation has {}",
                        rule.name,
                        rule.pat.len(),
                        hyp.len()
                    ),
                });
            }
            match_(&mut vars, &rule.pat, &hyp, call.loc)?;
            for (v, e) in rule.params.iter().zip(call.args.iter()) {
                vars.insert(v.clone(), e.clone());
            }
            let mut new = rule.rep.clone();
            replace(&vars, &mut new);

            if let Some(ret) = &call.ret {
                if !is_eq(&new, ret) {
                    return Err(Error {
                        loc: call.loc,
                        ty: ErrorTy::VerifError,
                        desc: format!("expected expression `{}`, found expression `{}`", ret, new),
                    });
                }
            }

            res.push(new);
        }

        if res.is_empty() {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!("expected expression `{}`, found nothing", a.rep,),
            });
        }

        if !((proof.is_empty() && res.iter().any(|h| is_eq(h, &a.rep)))
            || is_eq(&res.last().unwrap(), &a.rep))
        {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "expected expression `{}`, found expression `{}`",
                    a.rep,
                    res.last().unwrap()
                ),
            });
        }
        Ok(())
    }
}

fn insert_vars(m: &mut HashSet<String>, a: &Expr) {
    match a {
        Expr::Var { name, .. } => {
            m.insert(name.clone());
        }
        Expr::Func { args, .. } => args.iter().for_each(|a| insert_vars(m, a)),
    }
}

fn match_(m: &mut HashMap<String, Expr>, pat: &[Expr], e: &[Expr], loc: Loc) -> Result<(), Error> {
    for (pat, e) in pat.iter().zip(e.iter()) {
        match (pat, e) {
            (Expr::Var { name, .. }, _) => {
                if let Some(v) = m.get(name) {
                    if !is_eq(v, e) {
                        return Err(Error {
                            loc,
                            ty: ErrorTy::VerifError,
                            desc: format!("bindings not equal: {} != {}", v, e),
                        });
                    }
                }
                m.insert(name.clone(), e.clone());
            }

            (
                Expr::Func {
                    name: x, args: pb, ..
                },
                Expr::Func {
                    name: y, args: eb, ..
                },
            ) if x == y && pb.len() == eb.len() => {
                match_(m, pb, eb, loc)?;
            }

            (Expr::Func { .. }, ..) => {
                return Err(Error {
                    loc,
                    ty: ErrorTy::VerifError,
                    desc: format!("cannot match pattern {} with expression {}", pat, e),
                })
            }
        }
    }

    Ok(())
}

fn is_eq(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Var { name, .. }, Expr::Var { name: n2, .. }) => name == n2,
        (
            Expr::Func { name, args, .. },
            Expr::Func {
                name: n2, args: a2, ..
            },
        ) => name == n2 && args.iter().zip(a2.iter()).all(|(a, b)| is_eq(a, b)),
        _ => false,
    }
}

fn replace(m: &HashMap<String, Expr>, succed: &mut Expr) {
    match succed {
        Expr::Var { name, .. } => {
            *succed = m[name].clone();
        }
        Expr::Func { args, .. } => {
            for c in args.iter_mut() {
                replace(m, c);
            }
        }
    }
}

fn index(r: &[Expr], idxs: &[usize], loc: Loc) -> Result<Vec<Expr>, Error> {
    if let Some(i) = idxs.iter().find(|&&i| i >= r.len()) {
        Err(Error {
            loc,
            ty: ErrorTy::VerifError,
            desc: format!("invocation requires hypothetical with index {} while only {} hypothetical(s) have been inferred yet", i, r.len()),
        })
    } else {
        Ok(idxs
            .iter()
            .map(|&i| {
                let last = r.len() - 1;
                r[last - i].clone()
            })
            .collect())
    }
}

fn vars_to_string(v: Vec<String>) -> String {
    "`".to_string() + &v.join("`,`") + "`"
}
