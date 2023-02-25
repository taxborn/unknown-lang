use unknown_lang_parser::lexer::{state::Lexer, tokens::Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_eof_empty() {
        let mut lexer = Lexer::new("");
        let tok = lexer.lex_token();
        assert_eq!(tok, Token::Eof);
        let tok = lexer.lex_token();
        assert_eq!(tok, Token::Eof);
    }

    #[test]
    fn lexer_tokens() {
        let mut lexer = Lexer::new("!");

        let tok = lexer.lex_token();
        assert_eq!(tok, Token::Bang);

        let tok2 = lexer.lex_token();
        assert_eq!(tok2, Token::Eof);
    }

    #[test]
    fn lexes_triples_correctly() {
        let mut lexer = Lexer::new("<<<");

        let tok = lexer.lex_token();
        assert_eq!(tok, Token::LessLess);

        let tok2 = lexer.lex_token();
        assert_eq!(tok2, Token::Less);

        let tok3 = lexer.lex_token();
        assert_eq!(tok3, Token::Eof);
    }

    #[test]
    fn test_lexing_single_line_comments() {
        let input = "// This is a comment";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_token();
        assert!(matches!(tok, Token::Comment(false, _)));
    }

    #[test]
    fn test_single_line_comment_token_contents() {
        let input = "//test";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_token();
        assert_eq!(tok, Token::Comment(false, "test".to_string()));

        let tok = lexer.lex_token();
        assert_eq!(tok, Token::Eof);
    }

    #[test]
    fn test_lexing_multi_line_comments() {
        let input = "/*this is \n\na\n\nmultiline comment*/";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_token();
        assert!(matches!(tok, Token::Comment(true, _)));

        let tok = lexer.lex_token();
        assert_eq!(tok, Token::Eof);
    }
}
