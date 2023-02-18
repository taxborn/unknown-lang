//! Tokens for unknown-lang parser
pub enum Tokens {
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
    Str(String),
    // TODO: Numbers

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

    Eof
}
