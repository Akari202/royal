use logos::Logos;

mod parse;
mod lex;

const CLEAN: bool = true;
const NON_ERR: bool = true;

pub fn test() {
    test_lexer();
}

fn test_lexer() {
    // load NC/test.NC
    // let input = include_str!("../NC/rear_upright.NC");
    let input = include_str!("../NC/test.NC");
    let mut lexer = lex::Token::lexer(input);
    if CLEAN {
        loop {
            if let Some(token) = lexer.next() {
                if let Ok(token) = token {
                    if !NON_ERR {
                        continue;
                    } else if let lex::Token::LetterCode(_) = token {
                        // continue;
                        println!("Letter Code: {:?}", lexer.slice());
                    } else if token == lex::Token::EndOfBlock {
                        println!("{:?}", token);
                    } else if token == lex::Token::Comment {
                        println!("Comment: {:?}", lexer.slice());
                    } else {
                        println!("{:?}", token);
                    }
                } else {
                    println!("Error: {:?}", lexer.slice());
                }
            } else {
                break;
            }
        }
    } else {
        loop {
            let token = lexer.next();
            println!("{:?}", token);
            if token.is_none() {
                break;
            }
        }
    }
}
