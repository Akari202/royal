use std::error::Error;
use crate::gcode::parser::load_and_parse;

mod gcode;

fn main() -> Result<(), Box<dyn Error>> {
    load_and_parse("test.nc")?;
    Ok(())
}
