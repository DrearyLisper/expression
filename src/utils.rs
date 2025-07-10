use crate::types::{Expr, Factor, Term};

pub fn print(e: &Expr) {
    print_expr(e);
}

fn print_expr(e: &Expr) {
    match e {
        Expr::Term(t) => print_term(t),
        Expr::Add(l, r) => {
            print_term(l);
            print!("+");
            print_expr(r);
        }
        Expr::Sub(l, r) => {
            print_term(l);
            print!("-");
            print_expr(r);
        }
    }
}

fn print_term(t: &Term) {
    match t {
        Term::Factor(t) => print_factor(t),
        Term::Mul(l, r) => {
            print_factor(l);
            print!("*");
            print_term(r);
        }
        Term::Div(l, r) => {
            print_factor(l);
            print!("/");
            print_term(r);
        }
    }
}

fn print_factor(f: &Factor) {
    match f {
        Factor::Num(i) => {
            print!("{}", i);
        }
        Factor::Expr(e) => {
            print!("(");
            print_expr(e);
            print!(")");
        }
    }
}
