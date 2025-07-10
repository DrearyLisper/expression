pub enum Expr {
    Term(Box<Term>),
    Add(Box<Term>, Box<Expr>),
    Sub(Box<Term>, Box<Expr>),
}

pub enum Term {
    Factor(Box<Factor>),
    Mul(Box<Factor>, Box<Term>),
    Div(Box<Factor>, Box<Term>),
}

pub enum Factor {
    Num(i32),
    Expr(Box<Expr>),
}
