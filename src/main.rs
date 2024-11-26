use std::error::Error;
use xplorer::App;


fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;
   
    if let Err(err) = app.run() {
        eprintln!("{err}");
    }

    Ok(())
}
