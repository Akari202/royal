use std::error::Error;

mod gcode;

fn main() -> Result<(), Box<dyn Error>> {
    gcode::test();
    Ok(())
}
