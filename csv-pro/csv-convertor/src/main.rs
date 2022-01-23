use anyhow::{Result};
use std::error::Error;
use csv_lib::convertor::CsvAgent;
use std::path::PathBuf;
use std::ffi::OsString;

fn main() -> Result<(), Box<dyn Error>> {

    let file_path = OsString::from("../data/ADANIPORTS.csv");
    let out_path = OsString::from("../data/out/output.csv");

    let mut agent = CsvAgent::new(file_path, out_path);

    agent.csv_write()?;

    Ok(())
}
