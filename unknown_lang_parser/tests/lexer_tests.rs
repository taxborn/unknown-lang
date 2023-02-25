use unknown_lang_parser::lexer::{state::Lexer, tokens::Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_tokens() {
        let mut lexer = Lexer::new("!");
        let tok = lexer.lex_next_token();
        let tok2 = lexer.lex_next_token();

        assert_eq!(tok, Token::Bang);
        assert_eq!(tok2, Token::Eof);
    }

    #[test]
    fn lexes_triples_correctly() {
        let mut lexer = Lexer::new("<<<");
        let tok = lexer.lex_next_token();
        let tok2 = lexer.lex_next_token();
        let tok3 = lexer.lex_next_token();

        assert_eq!(tok, Token::LessLess);
        assert_eq!(tok2, Token::Less);
        assert_eq!(tok3, Token::Eof);
    }

    #[test]
    fn test_lexing_single_line_comments() {
        let input = "// This is a comment";
        let mut lexer = Lexer::new(input);
        let tok = lexer.lex_next_token();
        assert_eq!(tok, Token::Eof);
    }

    #[test]
    fn test_lexing_multi_line_comments() {
        let input = "/*this is \n\na\n\nmultiline comment*/";
        let mut lexer = Lexer::new(input);
        let tok = lexer.lex_next_token();
        assert_eq!(tok, Token::Eof);
    }
}
