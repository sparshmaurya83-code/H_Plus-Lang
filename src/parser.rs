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
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: &Token) {
        if std::mem::discriminant(self.peek())
            == std::mem::discriminant(expected)
        {
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

            Token::LBrace => {
                self.advance();
                Some(Stmt::Block(self.parse_block()?))
            }

            _ => None,
        }
    }

    fn parse_assignment(&mut self) -> Option<Stmt> {
        let name = match self.peek().clone() {
            Token::Identifier(n) => n,
            _ => return None,
        };

        self.advance();

        if matches!(self.peek(), Token::Equal) {
            self.advance();
            let value = self.parse_expr()?;
            return Some(Stmt::Assign(name, value));
        }

        None
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

    // ---------------- EXPRESSIONS ----------------

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.peek(), Token::EqualEqual) {
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::Binary(Box::new(expr), Operator::EqualEqual, Box::new(right));
        }

        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut expr = self.parse_term()?;

        while matches!(self.peek(), Token::Greater | Token::Less) {
            let op = match self.peek() {
                Token::Greater => Operator::Greater,
                Token::Less => Operator::Less,
                _ => unreachable!(),
            };

            self.advance();
            let right = self.parse_term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while matches!(self.peek(), Token::Plus) {
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::Binary(Box::new(expr), Operator::Plus, Box::new(right));
        }

        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        // function call support
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

            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen);
                Some(Expr::Grouping(Box::new(expr)))
            }

            _ => None,
        }
    }
}