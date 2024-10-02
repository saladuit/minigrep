use std::env;
mod config;
pub mod error;
pub use self::error::{Error, Result};

use crate::config::{Config, run};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    run(config)?;
    Ok(())
}
