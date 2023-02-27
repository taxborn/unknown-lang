use unknown_lang_parser::lexer::{state::Lexer, tokens::Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_eof_empty() {
        let mut lexer = Lexer::new("");
        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Eof);
        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Eof);
    }

    #[test]
    fn lexer_tokens() {
        let mut lexer = Lexer::new("!");

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Bang);

        let tok2 = lexer.lex_next();
        assert_eq!(tok2, Token::Eof);
    }

    #[test]
    fn lexes_triples_correctly() {
        let mut lexer = Lexer::new("<<<");

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::LessLess);

        let tok2 = lexer.lex_next();
        assert_eq!(tok2, Token::Less);

        let tok3 = lexer.lex_next();
        assert_eq!(tok3, Token::Eof);
    }


    #[test]
    fn test_getting_next_skips_comments() {
        let input = "//test\n+";
        let mut lexer = Lexer::new(input);

        let tok = lexer.get_next_token();
        assert!(matches!(tok, Token::Plus))
    }

    #[test]
    fn test_getting_next_skips_multiline_comments() {
        let input = "/*test\n*/\n+";
        let mut lexer = Lexer::new(input);

        let tok = lexer.get_next_token();
        assert!(matches!(tok, Token::Plus))
    }
}
