use anyhow::Result;
use csv;
use std::{collections::HashMap};
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use std::env;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Mydata {
    id: String,
    date: NaiveDate,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    last: f64,
    volume: f64,
}

pub struct CsvAgent {
    conn: String,
    data_store: HashMap<String, Mydata>,
}

impl CsvAgent {
    pub fn new() -> Self {
        Self {
            conn: "../../data/ADANIPORTS.csv".to_string(),
            data_store: HashMap::new(),
        }
    }

    pub fn csv_load(&mut self) -> Result<()> {
        let file = File::open(self.conn.clone())?;
        let mut rdr = csv::Reader::from_reader(file);
        for res in rdr.deserialize() {
            let record = res?;
            println!("resutl: {:?}", record);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //#[ignore]
    fn check_csv() {
        
    }
}