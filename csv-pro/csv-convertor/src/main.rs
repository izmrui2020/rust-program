use anyhow::Result;
use csv_lib::convertor::CsvAgent;

fn main() -> Result<()> {
    let mut agent = CsvAgent::new();

    agent.csv_load()?;

    Ok(())
}
