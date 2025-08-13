use nom::{IResult, Parser, branch::alt, bytes::tag, multi::many0};

use crate::types::{Expr, Factor, Term};

pub fn parse(e: &str) -> IResult<&str, Box<Expr>> {
    parse_expr(e)
}

fn spaces(s: &str) -> IResult<&str, Vec<&str>> {
    many0(tag(" ")).parse(s)
}

fn parse_expr(e: &str) -> IResult<&str, Box<Expr>> {
    let (e, l) = parse_term(e)?;

    let add = ((spaces, tag("+"), spaces, parse_expr)).parse(e);
    if add.is_ok() {
        let (e, (_, _, _, r)) = add.unwrap();
        return Ok((e, Box::new(Expr::Add(l, r))));
    }

    let sub = ((spaces, tag("-"), spaces, parse_expr)).parse(e);
    if sub.is_ok() {
        let (e, (_, _, _, r)) = sub.unwrap();
        return Ok((e, Box::new(Expr::Sub(l, r))));
    }

    Ok((e, Box::new(Expr::Term(l))))
}

fn parse_term(t: &str) -> IResult<&str, Box<Term>> {
    let (t, l) = parse_factor(t)?;

    let mul = ((spaces, tag("*"), spaces, parse_term)).parse(t);
    if mul.is_ok() {
        let (e, (_, _, _, r)) = mul.unwrap();
        return Ok((e, Box::new(Term::Mul(l, r))));
    }

    let div = ((spaces, tag("/"), spaces, parse_term)).parse(t);
    if div.is_ok() {
        let (e, (_, _, _, r)) = div.unwrap();
        return Ok((e, Box::new(Term::Div(l, r))));
    }

    Ok((t, Box::new(Term::Factor(l))))
}

use nom::character::complete::i32;

fn parse_factor_num(f: &str) -> IResult<&str, Box<Factor>> {
    let (f, num) = i32.parse(f)?;

    Ok((f, Box::new(Factor::Num(num))))
}

fn parse_factor_expr(f: &str) -> IResult<&str, Box<Factor>> {
    let (f, (_, _, expr, _, _)) = (tag("("), spaces, parse_expr, spaces, tag(")")).parse(f)?;

    Ok((f, Box::new(Factor::Expr(expr))))
}

fn parse_factor(f: &str) -> IResult<&str, Box<Factor>> {
    alt((parse_factor_num, parse_factor_expr)).parse(f)
}
