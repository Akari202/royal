use logos::Logos;
use crate::gcode::lex::Lexer;
use crate::gcode::preprocess::semicolon;

mod lex;
mod points;
mod preprocess;
mod filter;

pub fn test() {
    test_lexer();
}

fn test_lexer() {
    // load NC/test.NC
    // let input = include_str!("../NC/rear_upright.NC");
    let input = include_str!("../NC/test_arc.NC");
    let mut lexer = Lexer::new(input);
    let mut program = points::Program::from_file(input).unwrap();
    filter::filter(&mut program);
    // println!("{}", program);
    // println!("{}", program);
    // loop {
    //     if let Some(token) = lexer.next() {
    //         if let Ok(token) = token {
    //             println!("{:?}", token);
    //         }
    //     } else {
    //         break;
    //     }
    // }
}
