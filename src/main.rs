use std::{env, time::SystemTime, error::Error};

use xplorer::App;
use log::LevelFilter;

fn main() -> Result<(), Box<dyn Error>> {
    let log_level = env::var("LOG").map_or(0, |s| s.parse().unwrap());
    setup_logger(log_level)?;   
    
    let app = App::new()?;
   
    if let Err(err) = app.run() {
        eprintln!("{err}");
    }

    Ok(())
}

fn setup_logger(level: u64) -> Result<(), fern::InitError> {    
    let mut dispatch = fern::Dispatch::new();

    dispatch = match level {
        0 => dispatch.level(LevelFilter::Warn),
        1 => dispatch
                .level(LevelFilter::Warn)
                .level_for("xplorer", LevelFilter::Info),
        2 => dispatch
                .level(LevelFilter::Info)
                .level_for("xplorer", LevelFilter::Debug),
        3 => dispatch
                .level(LevelFilter::Debug)
                .level_for("xplorer", LevelFilter::Trace),
        _ => dispatch.level(log::LevelFilter::Trace),
    };

    let format = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {} ({}): {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("program.log")?)
        .chain(std::io::stdout());

    dispatch
        .chain(format)
        .apply()?; 

    Ok(())
}
