use std::error::Error;
use logos::Logos;

mod lex;
mod program;
mod filter;
mod preprocessor;
mod transform;

pub fn test() -> Result<(), Box<dyn Error>> {
    // load NC/test.NC
    let input = include_str!("../NC/rear_upright.NC");
    // let input = include_str!("../NC/test_arc.NC");
    // let input = include_str!("../NC/test.NC");
    // let input = include_str!("../NC/brakes.gcode");
    // let input = include_str!("../NC/Purple Worm Fewer supports+gcode - 4760012/files/PW_pt1.gcode");
    let mut program = program::Program::from_str(input)?;
    dbg!(program);

    // filter::filter(&mut program);
    // program.to_file("./NC/test_arc_filtered.NC".into()).unwrap();

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
    Ok(())
}
