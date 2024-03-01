use logos::Logos;
use crate::gcode::lex::Lexer;
use crate::gcode::preprocessor::semicolon;

mod lex;
mod points;
mod filter;
mod preprocessor;

pub fn test() {
    test_lexer();
}

fn test_lexer() {
    // load NC/test.NC
    // let input = include_str!("../NC/rear_upright.NC");
    let input = include_str!("../NC/test_arc.NC");
    // semicolon("./NC/Purple Worm Fewer supports+gcode - 4760012/files/PW_pt1.gcode".into()).unwrap();
    // let input = include_str!("../NC/Purple Worm Fewer supports+gcode - 4760012/files/PW_pt1.gcode");
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
