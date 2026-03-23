#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Say,
    Let,
    When,
    Otherwise,
    Use,

    Identifier(String),
    String(String),
    Number(f64),

    Plus,
    Equal,
    EqualEqual,
    Greater,
    Less,

    Dot,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,

    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // Skip whitespace
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
            }

            // Single-character tokens
            '{' => { tokens.push(Token::LBrace); chars.next(); }
            '}' => { tokens.push(Token::RBrace); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '.' => { tokens.push(Token::Dot); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '>' => { tokens.push(Token::Greater); chars.next(); }
            '<' => { tokens.push(Token::Less); chars.next(); }

            // = and ==
            '=' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::EqualEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }

            // Strings
            '"' => {
                chars.next(); // skip opening quote
                let mut value = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        break;
                    }
                    value.push(ch);
                    chars.next();
                }

                chars.next(); // closing quote
                tokens.push(Token::String(value));
            }

            // Numbers
            '0'..='9' => {
                let mut number = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() || ch == '.' {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Ok(n) = number.parse::<f64>() {
                    tokens.push(Token::Number(n));
                }
            }

            // Identifiers & keywords
            _ => {
                let mut ident = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let token = match ident.as_str() {
                    "say" => Token::Say,
                    "let" => Token::Let,
                    "when" => Token::When,
                    "otherwise" => Token::Otherwise,
                    "use" => Token::Use,
                    _ => Token::Identifier(ident),
                };

                tokens.push(token);
            }
        }
    }

    tokens.push(Token::EOF);
    tokens
}