use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: &Token) {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(expected) {
            self.advance();
        } else {
            panic!("Expected {:?}, got {:?}", expected, self.peek());
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];

        while !matches!(self.peek(), Token::EOF) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.advance();
            }
        }

        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::Fn => self.parse_function(),
            Token::Return => self.parse_return(),
            Token::Say => {
                self.advance();
                Some(Stmt::Say(self.parse_expr()?))
            }
            Token::Let => {
                self.advance();
                let name = match self.peek().clone() {
                    Token::Identifier(n) => {
                        self.advance();
                        n
                    }
                    _ => return None,
                };
                self.expect(&Token::Equal);
                Some(Stmt::Let(name, self.parse_expr()?))
            }
            Token::Identifier(_) => self.parse_assignment(),
            Token::LBrace => Some(Stmt::Block(self.parse_block()?)),
            Token::When => self.parse_when(),
            _ => None,
        }
    }

    fn parse_function(&mut self) -> Option<Stmt> {
        self.advance();

        let name = match self.peek().clone() {
            Token::Identifier(n) => {
                self.advance();
                n
            }
            _ => return None,
        };

        self.expect(&Token::LParen);

        let mut params = vec![];
        while !matches!(self.peek(), Token::RParen) {
            if let Token::Identifier(p) = self.peek().clone() {
                params.push(p);
                self.advance();
            }

            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }

        self.expect(&Token::RParen);
        self.expect(&Token::LBrace);

        let body = self.parse_block()?;

        Some(Stmt::Function { name, params, body })
    }

    fn parse_return(&mut self) -> Option<Stmt> {
        self.advance();

        if matches!(self.peek(), Token::RBrace) {
            return Some(Stmt::Return(None));
        }

        Some(Stmt::Return(Some(self.parse_expr()?)))
    }

    fn parse_assignment(&mut self) -> Option<Stmt> {
        let name = match self.peek().clone() {
            Token::Identifier(n) => n,
            _ => return None,
        };

        self.advance();
        self.expect(&Token::Equal);

        Some(Stmt::Assign(name, self.parse_expr()?))
    }

    fn parse_when(&mut self) -> Option<Stmt> {
        self.advance();
        let cond = self.parse_expr()?;

        self.expect(&Token::LBrace);
        let then_block = Stmt::Block(self.parse_block()?);

        Some(Stmt::When(cond, Box::new(then_block), None))
    }

    fn parse_block(&mut self) -> Option<Vec<Stmt>> {
        let mut stmts = vec![];

        while !matches!(self.peek(), Token::RBrace | Token::EOF) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.advance();
            }
        }

        self.expect(&Token::RBrace);
        Some(stmts)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        if matches!(self.peek(), Token::LParen) {
            expr = self.finish_call(expr)?;
        }

        Some(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Option<Expr> {
        self.expect(&Token::LParen);

        let mut args = vec![];
        while !matches!(self.peek(), Token::RParen) {
            args.push(self.parse_expr()?);

            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }

        self.expect(&Token::RParen);

        Some(Expr::Call {
            callee: Box::new(callee),
            args,
        })
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Some(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Some(Expr::String(s))
            }
            Token::Identifier(name) => {
                self.advance();
                Some(Expr::Variable(name))
            }
            _ => None,
        }
    }
}