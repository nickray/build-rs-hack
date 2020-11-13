use std::io::Write as _;
use std::{error, fs, io};

#[derive(serde::Deserialize)]
struct Config {
    parameters: Parameters,
}

#[derive(serde::Deserialize)]
struct Parameters {
    dimension: usize,
    number: u8,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("cargo:rerun-if-changed=config/src/lib.rs");
    println!("cargo:rerun-if-changed=cfg.toml");

    let config = fs::read_to_string("cfg.toml")?;
    let config: Config = toml::from_str(&config)?;

    let mut file = io::BufWriter::new(fs::File::create("config/src/lib.rs")?);
    writeln!(
        &mut file,
        "pub const DIMENSION: usize = {};",
        config.parameters.dimension
    )?;
    writeln!(
        &mut file,
        "pub const NUMBER: u8 = {};",
        config.parameters.number
    )?;

    Ok(())
}
