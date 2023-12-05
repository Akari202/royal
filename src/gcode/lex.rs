use logos::{Logos, SpannedIter};
use crate::gcode::lex::tokens::Token;

pub mod tokens;
pub type Spanned<Token, Location, Error> = Result<(Location, Token, Location), Error>;

pub enum LexicalError {
    InvalidToken
}

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { token_stream: Token::lexer(input).spanned() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            match token {
                Err(_) => Err(LexicalError::InvalidToken),
                Ok(_) => Ok((span.start, token.unwrap(), span.end)),
            }
        })
    }
}
