use unknown_lang_parser::lexer::{state::Lexer, tokens::Token};

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to get all the tokens at once within the Lexer
    fn get_toks<'a>(lexer: &mut Lexer) -> Vec<Token> {
        let mut toks: Vec<Token> = vec![];

        loop {
            let tok = lexer.lex_next();

            if tok == Ok(Token::Eof) {
                break;
            }

            toks.push(tok.unwrap());
        }

        toks
    }

    #[test]
    fn lexer_eof_empty() {
        let mut lexer = Lexer::new("");
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn lexer_tokens() {
        let mut lexer = Lexer::new("!");

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Bang));

        let tok2 = lexer.lex_next();
        assert_eq!(tok2, Ok(Token::Eof));
    }

    #[test]
    fn lexes_triples_correctly() {
        let input = "<<<>>>";
        let mut lexer = Lexer::new(input);
        let toks = get_toks(&mut lexer);
        let expected = vec![
            Token::LessLess,
            Token::Less,
            Token::GreaterGreater,
            Token::Greater,
        ];
        assert_eq!(toks, expected);

        let exp_eof = lexer.lex_next();
        assert_eq!(exp_eof, Ok(Token::Eof));
    }

    #[test]
    fn test_assignment() {
        let input = "let a := 5;";
        let mut lexer = Lexer::new(input);

        let toks = get_toks(&mut lexer);

        let expected = vec![
            Token::Ident("let".to_string()),
            Token::Ident("a".to_string()),
            Token::Colon,
            Token::Eq,
            Token::Number(10, "5".to_string()),
            Token::Semi,
        ];

        assert_eq!(toks, expected);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

}
