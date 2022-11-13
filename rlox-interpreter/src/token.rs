use alloc::string::String as RustString;

#[derive(Debug, Clone)]
pub enum TokenType {
    /// '('
    LeftParen,
    /// ')'
    RightParen,
    /// '{'
    LeftBrace,
    /// '}'
    RightBrace,
    /// ','
    Comma,
    /// '.'
    Dot,
    /// '-'
    Minus,
    /// '+'
    Plus,
    /// ':'
    SemiColon,
    /// '/'
    Slash,
    /// '*'
    Star,
    /// '!'
    Bang,
    /// '!='
    BangEqual,
    /// =
    Equal,
    /// ==
    EqualEqual,
    /// >
    Greater,
    /// >=
    GreaterEqual,
    /// <
    Less,
    /// <=
    LessEqual,
    Identifier,
    /// "", string types
    String,
    /// 123, numeric types
    Number,
    /// and
    And,
    /// class
    Class,
    /// else
    Else,
    /// false
    False,
    /// for
    For,
    /// if
    If,
    /// nil
    Nil,
    /// or
    Or,
    /// print
    Print,
    /// super
    Super,
    /// this
    This,
    /// var
    Var,
    /// while
    While,
    /// eof
    Eof,
    /// return
    Return,
    /// true
    True,
}

pub fn try_to_keyword(text: &str) -> Option<TokenType> {
    match text {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Float(f64),
    String(RustString),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme:     RustString,
    pub literal:    Option<Literal>,
    pub line:       usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: RustString,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn stringify(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
