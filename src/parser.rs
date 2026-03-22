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
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        self.pos += 1;
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
            Token::Say => {
                self.advance();
                Some(Stmt::Say(self.parse_expr()?))
            }

            Token::Let => {
                self.advance();
                if let Token::Identifier(name) = self.peek().clone() {
                    self.advance();
                    self.advance(); // =
                    Some(Stmt::Let(name, self.parse_expr()?))
                } else {
                    None
                }
            }

            Token::Use => {
                self.advance();
                if let Token::Identifier(name) = self.peek().clone() {
                    self.advance();
                    Some(Stmt::Use(name))
                } else {
                    None
                }
            }

            Token::When => self.parse_when(),

            _ => None,
        }
    }

    fn parse_when(&mut self) -> Option<Stmt> {
        self.advance();
        let condition = self.parse_expr()?;

        self.advance(); // {

        let mut then_block = vec![];
        while !matches!(self.peek(), Token::RBrace) {
            if let Some(stmt) = self.parse_stmt() {
                then_block.push(stmt);
            } else {
                self.advance();
            }
        }

        self.advance(); // }

        let mut else_block = None;

        if matches!(self.peek(), Token::Otherwise) {
            self.advance();
            self.advance(); // {

            let mut block = vec![];
            while !matches!(self.peek(), Token::RBrace) {
                if let Some(stmt) = self.parse_stmt() {
                    block.push(stmt);
                } else {
                    self.advance();
                }
            }

            self.advance();
            else_block = Some(block);
        }

        Some(Stmt::When(condition, then_block, else_block))
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let left = self.parse_primary()?;

        if let Token::Plus = self.peek() {
            self.advance();
            let right = self.parse_primary()?;
            return Some(Expr::Binary(Box::new(left), Operator::Plus, Box::new(right)));
        }

        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek().clone() {
            Token::Identifier(name) => {
                self.advance();

                if matches!(self.peek(), Token::Dot) {
                    self.advance();

                    if let Token::Identifier(func) = self.peek().clone() {
                        self.advance();

                        self.advance(); // (

                        let mut args = vec![];

                        while !matches!(self.peek(), Token::RParen) {
                            if let Some(arg) = self.parse_expr() {
                                args.push(arg);
                            } else {
                                self.advance();
                            }
                        }

                        self.advance(); // )

                        return Some(Expr::Call {
                            module: name,
                            function: func,
                            args,
                        });
                    }
                }

                Some(Expr::Variable(name))
            }

            Token::String(s) => {
                self.advance();
                Some(Expr::String(s))
            }

            Token::Number(n) => {
                self.advance();
                Some(Expr::Number(n))
            }

            _ => None,
        }
    }
}