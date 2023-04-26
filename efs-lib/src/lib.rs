pub mod config;
pub mod parser;
pub mod project;

#[cfg(test)]
mod tests {
    use crate::parser::{lexer::Lexer, token::TokenType};

    #[test]
    fn lexer() {
        let text = String::from(
            "
            fn test(x: int) {
                let x: float = 2.;
                x = 5. + x;
                say(test);
            }
        ",
        );

        let mut lex = Lexer::new(text);
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
                }
            }
        }
        
        assert!(!failed);
        assert!(false);
        //assert_eq!(3, 4);
    }
}
