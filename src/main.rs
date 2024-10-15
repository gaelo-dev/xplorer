use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = xplorer::run() {
        eprintln!("{err}");
    }

    Ok(())
}
