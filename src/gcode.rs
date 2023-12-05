use logos::Logos;

mod parse;
mod lex;
mod ast;

const CLEAN: bool = true;
const NON_ERR: bool = true;

pub fn test() {
    test_lexer();
    // test_lexer_parser();
}

fn test_lexer_parser() {
    // let input = include_str!("../NC/rear_upright.NC");
    let input = include_str!("../NC/test.NC");
    let lexer = lex::Lexer::new(&input[..]);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer)?;

    println!("{:?}", ast);
}

fn test_lexer() {
    // load NC/test.NC
    // let input = include_str!("../NC/rear_upright.NC");
    let input = include_str!("../NC/test.NC");
    let mut lexer = lex::tokens::Token::lexer(input);
    if CLEAN {
        loop {
            if let Some(token) = lexer.next() {
                if let Ok(token) = token {
                    if !NON_ERR {
                        continue;
                    } else if let lex::tokens::Token::LetterCode(_) = token {
                        // continue;
                        println!("Letter Code: {:?}", lexer.slice());
                    } else if token == lex::tokens::Token::EndOfBlock {
                        println!("{:?}", token);
                    } else if token == lex::tokens::Token::Comment {
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
