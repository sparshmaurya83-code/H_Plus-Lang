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
    LParen,
    RParen,
    LBrace,
    RBrace,

    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    for word in input.split_whitespace() {
        let token = match word {
            "say" => Token::Say,
            "let" => Token::Let,
            "when" => Token::When,
            "otherwise" => Token::Otherwise,
            "use" => Token::Use,

            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "." => Token::Dot,

            "+" => Token::Plus,
            "==" => Token::EqualEqual,
            "=" => Token::Equal,
            ">" => Token::Greater,
            "<" => Token::Less,

            _ => {
                if word.starts_with("\"") && word.ends_with("\"") {
                    Token::String(word.trim_matches('"').to_string())
                } else if let Ok(n) = word.parse::<f64>() {
                    Token::Number(n)
                } else {
                    Token::Identifier(word.to_string())
                }
            }
        };

        tokens.push(token);
    }

    tokens.push(Token::EOF);
    tokens
}