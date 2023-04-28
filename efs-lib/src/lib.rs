pub mod config;
pub mod parser;
pub mod project;

#[cfg(test)]
mod tests {
    use crate::parser::{
        lexer::Lexer,
        token::{self, Token, TokenType},
    };

    static TEST_TEXT: &str = "
        const val: string = \"hello world\";

        fn test(x: int) {
            let x: float = 2.;
            x = 5. + x;
            say(test);
        }
    ";

    #[test]
    fn token() {
        let text = String::from("i8");
        let mut lex = Lexer::new(text);

        println!(
            "All Keywords: {:#?}",
            enum_iterator::all::<token::Keyword>().collect::<Vec<_>>()
        );

        println!("{}", token::Keyword::TypeName(token::TypeName::Byte));

        assert_eq!(
            lex.next_token().unwrap().token,
            Token::Keyword(token::Keyword::TypeName(token::TypeName::Byte))
        )
    }
    #[test]
    fn lexer() {
        let mut lex = Lexer::new(String::from(TEST_TEXT));
        let mut failed = false;
        loop {
            match lex.next_token() {
                Ok(token) => {
                    println!("{:?}", token);
                    if token.token.token_type() == TokenType::EOI {
                        break;
                    }
                }
                Err(pos) => {
                    println!("unknown at {}", pos);
                    failed = true;
                    break;
                }
            }
        }

        assert!(!failed);
        //assert!(false);
    }
}
