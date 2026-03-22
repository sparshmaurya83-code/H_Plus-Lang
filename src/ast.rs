#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Variable(String),

    Call {
        module: String,
        function: String,
        args: Vec<Expr>,
    },

    Binary(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Say(Expr),
    Let(String, Expr),
    When(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    Use(String),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    EqualEqual,
    Greater,
    Less,
}