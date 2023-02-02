use std::fmt;
// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust

pub enum TokenType {
    UNKNOWN_TOKEN,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    INTEGER_LITERAL,
}

impl TokenType {
    fn value(&self) -> &str {
        match *self {
            TokenType::UNKNOWN_TOKEN => "unknown token",
            TokenType::PLUS => "+",
            TokenType::MINUS => "-",
            TokenType::STAR => "*",
            TokenType::SLASH => "/",
            TokenType::INTEGER_LITERAL => "integer literal",
        }
    }
}

// ttype because type is a reserved word

pub struct Token {
    ttype: TokenType,
    value: i32,
}

impl Token {
    /// Stores Token data
    /// Args:
    ///     ttype: Option<TokenType>: Type of Token to instantiate. None results in TokenType::UNKNOWN_TOKEN
    ///     value: Option<i32>: i32 value of Token to instantiate. None results in 0.
    pub fn new(ttype: Option<TokenType>, value: Option<i32>) -> Token {
        Token {
            ttype: match ttype {
                Some(ttype) => ttype,
                None => TokenType::UNKNOWN_TOKEN,
            },
            value: match value {
                Some(value) => value,
                None => 0,
            },
        }
    }
}
// implement display for it, so it can be printed
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            //"Token:\n\tTYPE = {} {}{}",
            "Token:\n\tTYPE = [{}] {}",
            &self.ttype.value(), // Type of token
            // TODO: Figure this out self.ttype as i32,   // Index of value in enum
            match &self.ttype {
                TokenType::INTEGER_LITERAL => format!("\n\tVALUE = {}", &self.value),
                _ => String::new(),
            }
        )
    }
}
