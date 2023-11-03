use std::fmt::Display;

use crate::{
    error::{Error, Loc},
    token::{Scanner, TokenTy},
};

#[derive(Clone, Debug)]
pub struct Rule {
    pub name: String,
    pub loc: Loc,
    pub pat: Vec<Expr>,
    pub rep: Expr,
    pub params: Vec<String>,
    pub proof: Option<Vec<Call>>,
}

#[derive(Clone, Debug)]
pub struct Call {
    pub f: String,
    pub loc: Loc,
    pub idxs: Vec<usize>,
    pub args: Vec<Expr>,
    pub ret: Option<Expr>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Var {
        name: String,
        loc: Loc,
    },
    Func {
        loc: Loc,
        name: String,
        args: Vec<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Func { name, args, .. } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Expr::Var { name, .. } => write!(f, "{}", name),
        }
    }
}

pub struct Parser<'a> {
    sc: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(sc: Scanner<'a>) -> Self {
        Self { sc }
    }
    pub fn parse_def(&mut self) -> Result<Option<Rule>, Error> {
        if self.sc.peek()?.ty() == TokenTy::Eof {
            return Ok(None);
        }
        let tok = self.sc.expect_one(&[TokenTy::Axiom, TokenTy::Theorem])?;
        match tok.ty() {
            TokenTy::Axiom => self.parse_axiom(),
            TokenTy::Theorem => self.parse_theorem(),
            _ => unreachable!(),
        }
        .map(Some)
    }

    fn parse_axiom(&mut self) -> Result<Rule, Error> {
        Ok(self.parse_rule_basic()?)
    }

    fn parse_theorem(&mut self) -> Result<Rule, Error> {
        let rule = self.parse_rule_basic()?;
        let proof = Some(self.parse_proof()?);

        Ok(Rule { proof, ..rule })
    }

    fn parse_rule_basic(&mut self) -> Result<Rule, Error> {
        let (loc, name) = self.sc.expect_identifier()?;

        self.sc.expect_token(TokenTy::DoubleColon)?;

        let mut params = Vec::new();
        if self.sc.is_token(TokenTy::Lparen)? && !self.sc.is_token(TokenTy::Rparen)? {
            loop {
                let (_loc, name) = self.sc.expect_identifier()?;
                params.push(name);
                if self.expect_commma_or(TokenTy::Rparen)? {
                    break;
                }
            }

            self.sc.expect_token(TokenTy::DoubleColon)?;
        }

        self.sc.expect_token(TokenTy::Lbrace)?;
        let mut pat = Vec::new();

        if !self.sc.is_token(TokenTy::Turnstile)? {
            loop {
                let expr = self.parse_expr()?;
                pat.push(expr);
                if self.expect_commma_or(TokenTy::Turnstile)? {
                    break;
                }
            }
        }

        let rep = self.parse_expr()?;
        self.sc.expect_token(TokenTy::Rbrace)?;

        Ok(Rule {
            name,
            loc,
            pat,
            rep,
            params,
            proof: None,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let (loc, name) = self.sc.expect_identifier()?;
        if name.chars().next().unwrap().is_uppercase() {
            return Ok(Expr::Var { name, loc });
        }
        let mut args = Vec::new();
        if self.sc.is_token(TokenTy::Lparen)? && !self.sc.is_token(TokenTy::Rparen)? {
            loop {
                let s = self.parse_expr()?;
                args.push(s);
                if self.expect_commma_or(TokenTy::Rparen)? {
                    break;
                }
            }
        }

        Ok(Expr::Func { name, args, loc })
    }

    fn parse_proof(&mut self) -> Result<Vec<Call>, Error> {
        self.sc.expect_token(TokenTy::Lcurly)?;

        let mut calls = Vec::new();
        while self.sc.peek()?.ty() != TokenTy::Rcurly {
            calls.push(self.parse_call()?);
        }

        self.sc.expect_token(TokenTy::Rcurly)?;

        Ok(calls)
    }

    fn parse_call(&mut self) -> Result<Call, Error> {
        let (loc, name) = self.sc.expect_identifier()?;

        let mut idxs = Vec::new();
        if self.sc.is_token(TokenTy::Lbrace)? && !self.sc.is_token(TokenTy::Rbrace)? {
            loop {
                let (_l, s) = self.sc.expect_number()?;
                idxs.push(s);
                if self.expect_commma_or(TokenTy::Rbrace)? {
                    break;
                }
            }
        }

        let mut args = Vec::new();
        if self.sc.is_token(TokenTy::Lparen)? && !self.sc.is_token(TokenTy::Rparen)? {
            loop {
                let s = self.parse_expr()?;
                args.push(s);
                if self.expect_commma_or(TokenTy::Rparen)? {
                    break;
                }
            }
        }

        let ret = if self.sc.is_token(TokenTy::Turnstile)? {
            let e = self.parse_expr()?;
            Some(e)
        } else {
            None
        };

        Ok(Call {
            f: name,
            loc,
            idxs,
            args,
            ret,
        })
    }

    fn expect_commma_or(&mut self, b: TokenTy) -> Result<bool, Error> {
        let mut res = self.sc.expect_one(&[TokenTy::Comma, b.clone()])?.ty();
        if res == TokenTy::Comma {
            res = self.sc.peek()?.ty();
            if res == b {
                self.sc.expect_token(b.clone())?;
            }
        }

        Ok(res == b)
    }
}
