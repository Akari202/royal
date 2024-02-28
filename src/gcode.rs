use logos::Logos;

mod lex;
mod points;

pub fn test() {
    test_lexer();
}

fn test_lexer() {
    // load NC/test.NC
    // let input = include_str!("../NC/rear_upright.NC");
    let input = include_str!("../NC/test_arc.NC");
    let mut lexer = lex::tokens::Token::lexer(input);
    let program = points::Program::from_lex(&mut lexer);
    println!("{:?}", program);
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
