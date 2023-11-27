use std::error::Error;
use nom::{bytes, IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use crate::gcode::Program;

pub fn load_and_parse(file: impl AsRef<std::path::Path>) -> Result<Program, Box<dyn Error>> {
    let code: String = std::fs::read_to_string(file)?;
    Ok(parse(code)?)
}

fn parse(code: String) -> Result<Program, Box<dyn Error>> {
    let mut program = Program::new();
    
    let mut code = code.trim_start_matches("%").trim();
    let mut token: &str = "";
    for _ in 0..10 {
        (code, token) = parse_token(code).unwrap();
        println!("{:?}", token);
    }


    Ok(program)
}

fn parse_token(input: &str) -> IResult<&str, &str> {
    let input = input.trim_start();
    nom::bytes::complete::take_while(|c: char| c.is_alphanumeric())(input)
}

fn parse_token_key(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(|c: char| c.is_alphabetic())(input)
}
