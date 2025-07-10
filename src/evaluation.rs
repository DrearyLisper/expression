use crate::types::{Expr, Factor, Term};

pub fn eval(e: &Expr) -> i32 {
    eval_expr(e)
}

fn eval_expr(e: &Expr) -> i32 {
    match e {
        Expr::Term(t) => eval_term(t),
        Expr::Add(l, r) => eval_term(l) + eval_expr(r),
        Expr::Sub(l, r) => eval_term(l) - eval_expr(r),
    }
}

fn eval_term(t: &Term) -> i32 {
    match t {
        Term::Factor(t) => eval_factor(t),
        Term::Mul(l, r) => eval_factor(l) * eval_term(r),
        Term::Div(l, r) => eval_factor(l) / eval_term(r),
    }
}

fn eval_factor(f: &Factor) -> i32 {
    match f {
        Factor::Num(i) => *i,
        Factor::Expr(e) => eval_expr(e),
    }
}
