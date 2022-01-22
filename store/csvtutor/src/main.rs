use csv;
use std::error::Error;
use std::process;
use std::io;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b':')
        .from_reader(io::stdin());

    for res in rdr.records() {
        let record = res?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("err {}", &err);
        process::exit(1);
    }
}
