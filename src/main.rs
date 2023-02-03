use std::io;
use ascii_clock::mytui;
fn main() -> Result<(), io::Error> {
    mytui::run()?;
    Ok(())
}
