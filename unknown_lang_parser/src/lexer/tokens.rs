//! Tokens for unknown-lang parser
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /// (
    LPar,
    /// )
    RPar,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// {
    LBrace,
    /// }
    RBrace,
    /// =
    Eq,
    /// ==
    EqEq,
    /// :
    Colon,
    /// :::
    ColonColon,
    /// ;
    Semi,
    /// =>
    FatArrow,
    /// $
    Dollar,
    /// ,
    Comma,
    /// ->
    RightArrow,
    /// .
    Dot,
    /// ..
    DotDot,
    /// ~
    Tilde,

    Char(char),
    // The first boolean is used to store if it a double-quoted string (true) or single-quoted
    // string (false)
    Str(bool, String),
    Ident(String),
    Comment(bool, String),
    // Currently all numbers (including floats) will go into this number
    // token, and later will be split into different typed numbers.
    Number(String),

    // Operators
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Percent,
    /// &
    Ampersand,
    /// |
    Bar,
    /// ^
    Hat,
    /// >
    Greater,
    /// >=
    GreaterEq,
    /// >>>
    GreaterGreater,
    /// <
    Less,
    /// <=
    LessEq,
    /// <<
    LessLess,
    /// !
    Bang,
    /// !=
    BangEq,

    /// +=
    PlusEq,

    Eof,
    Error(char),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::LPar => write!(f, "("),
            Token::RPar => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Eq => write!(f, "="),
            Token::EqEq => write!(f, "=="),
            Token::Colon => write!(f, ":"),
            Token::ColonColon => write!(f, "::"),
            Token::Semi => write!(f, ";"),
            Token::FatArrow => write!(f, "=>"),
            Token::Dollar => write!(f, "$"),
            Token::Comma => write!(f, ","),
            Token::RightArrow => write!(f, "->"),
            Token::Dot => write!(f, "."),
            Token::DotDot => write!(f, ".."),
            Token::Tilde => write!(f, "~"),

            Token::Char(chr) => write!(f, "'{chr}'"),
            Token::Str(true, string) => write!(f, "\"{string}\""),
            Token::Str(false, string) => write!(f, "'{string}'"),
            Token::Ident(ident) => write!(f, "[{ident}]"),
            Token::Comment(true, cmt) => write!(f, "{cmt}"),
            Token::Comment(false, cmt) => write!(f, "// {cmt}"),
            Token::Error(err) => write!(f, "ERR[{err}]"),
            Token::Number(num) => write!(f, "Num({num})"),

            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Ampersand => write!(f, "&"),
            Token::Bar => write!(f, "|"),
            Token::Hat => write!(f, "^"),
            Token::Greater => write!(f, ">"),
            Token::GreaterEq => write!(f, ">="),
            Token::GreaterGreater => write!(f, ">>"),
            Token::Less => write!(f, "<"),
            Token::LessEq => write!(f, "<="),
            Token::LessLess => write!(f, "<<"),
            Token::Bang => write!(f, "!"),
            Token::BangEq => write!(f, "!="),
            Token::PlusEq => write!(f, "+="),
            Token::Eof => write!(f, "<EOF>"),
        }
    }
}
