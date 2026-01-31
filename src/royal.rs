use std::error::Error;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(testing, "/royal/testing.rs");

pub fn test() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../Royal/test.royal");
    dbg!(testing::TermParser::new().parse("((22)"));
    Ok(())
}