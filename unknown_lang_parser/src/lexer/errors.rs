#[derive(Debug, PartialEq)]
pub enum LexingError {
    NoNextCharacter,
    UnclosedString,
    UnclosedMutlilineComment,
    UnusedEscape,
    UnknownBase(String),
    UnknownEscapedCharacter(char),
    BaseTooLarge(u8),
    UnclosedBaseSpecifier,
    UnknownCharacter(char),
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoNextCharacter => write!(f, "no next character"),
            Self::UnclosedString => write!(f, "unclosed string"),
            Self::UnclosedMutlilineComment => {
                write!(f, "unclosed multi-line comment")
            }
            Self::UnclosedBaseSpecifier => write!(f, "unclosed base specifier"),
            Self::UnusedEscape => write!(f, "unused escape sequence"),
            Self::UnknownCharacter(chr) => {
                write!(f, "unknown character encountered while lexing: {chr}")
            }
            Self::UnknownBase(base) => write!(f, "invalid base: {base}"),
            Self::BaseTooLarge(base) => write!(f, "unsupported base: {base}"),
            Self::UnknownEscapedCharacter(chr) => {
                write!(f, "unknown escape character: {chr}")
            }
        }
    }
}

impl std::error::Error for LexingError {}
