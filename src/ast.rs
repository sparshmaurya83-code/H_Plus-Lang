#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Variable(String),

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    Binary(Box<Expr>, Operator, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Say(Expr),
    Let(String, Expr),
    Assign(String, Expr),

    Block(Vec<Stmt>),

    When(Expr, Box<Stmt>, Option<Box<Stmt>>),

    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },

    Return(Option<Expr>),

    Use(String),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    EqualEqual,
    Greater,
    Less,
}