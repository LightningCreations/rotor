#![deny(warnings)]

use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();

    let _data = &fs::read(&args[1])?;

    Ok(())
}
