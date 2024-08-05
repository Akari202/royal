use std::error::Error;
use std::io::Write;

mod gcode;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(buf, "{}: {}", record.level(), record.args())
        })
        .init();
    // env_logger::init();
    gcode::test()?;
    Ok(())
}
