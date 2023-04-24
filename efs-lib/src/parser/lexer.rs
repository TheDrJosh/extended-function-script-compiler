use super::token::TokenHolder;



pub struct Lexer {
    text: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text: text.chars().collect(),
            pos: 0,
        }
    }


    pub fn next_token() -> anyhow::Result<Option<TokenHolder>> {
        
        

        todo!()
    }

}