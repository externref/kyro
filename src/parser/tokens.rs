#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    LeftBracket, RightBracket, Colon,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    Identifier, String, Number,
    And, Class, Else, False, Fn, For, If, Nil, Or,
    Echo, Return, Super, This, True, Var, While,
    Try, Catch, Throw, 
    Break, Continue,
    In, Ampersand, Pipe, Caret, Tilde,
    LessLess, GreaterGreater,
    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.r#type, self.lexeme, self.literal)
    }
}
