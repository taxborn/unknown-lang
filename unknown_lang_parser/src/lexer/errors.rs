#[derive(Debug, PartialEq)]
pub enum LexingError {
    NoNextCharacter,
    UnclosedString,
    UnclosedMutlilineComment,
    UnusedEscape,
    UnknownEscapedCharacter(char),
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoNextCharacter => write!(f, "no next character"),
            Self::UnclosedString => write!(f, "unclosed string"),
            Self::UnclosedMutlilineComment => write!(f, "unclosed multi-line comment"),
            Self::UnusedEscape => write!(f, "unused escape sequence"),
            Self::UnknownEscapedCharacter(chr) => write!(f, "unknown escape character: {chr}"),
        }
    }
}

impl std::error::Error for LexingError {}
