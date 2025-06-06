use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    built::write_built_file()?;
    Ok(())
}
