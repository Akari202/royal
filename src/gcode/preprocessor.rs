use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

// add a semicolon to the end of each line in the file, write and close the file
pub fn semicolon(path_buf: PathBuf) -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(&path_buf)?;
    let mut output = std::fs::File::create(&path_buf)?;
    for line in input.lines() {
        writeln!(output, "{};", line)?;
    }
    Ok(())
}
