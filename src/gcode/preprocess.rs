use std::error::Error;
use std::io::Write;

// add a semicolon to the end of each line in the file, write and close the file
pub fn semicolon(input: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create("../NC/semicolon.NC")?;
    for line in input.lines() {
        file.write_all(format!("{};\n", line).as_bytes())?;
    }
    file.flush()?;
    file.sync_all()?;
    Ok(())
}
