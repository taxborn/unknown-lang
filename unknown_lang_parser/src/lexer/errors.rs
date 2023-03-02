#[derive(Debug, PartialEq)]
pub enum LexingError {
    NoNextCharacter,
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::NoNextCharacter => write!(f, "no next character"),
        }
    }
}

impl std::error::Error for LexingError {}
