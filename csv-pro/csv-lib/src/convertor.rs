use std::error::Error;
use csv;
use std::{collections::HashMap};
use bigdecimal::{BigDecimal, FromPrimitive};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use std::fs::File;
use std::ffi::OsString;

#[derive(Debug, Default, Clone)]
struct Mydata {
    id: String,
    date: String,
    open: BigDecimal,
    prev_close: BigDecimal,
    close: BigDecimal,
    high: BigDecimal,
    low: BigDecimal,
    last: BigDecimal,
    volume: BigDecimal,
}

// define proto
#[derive(Debug, Serialize)]
struct OutPutData {
    id: String,
    prev_22: f64,
}

#[derive(Debug, Deserialize)]
struct Hogedata {
    id: String,
    date: String,
    open: String,
    prev_close: String,
    close: String,
    high: String,
    low: String,
    last: String,
    volume: String,
}

pub struct CsvAgent {
    conn: OsString,
    out: OsString,
    my_data: Mydata,
    data_store: HashMap<String, Mydata>,
}

impl CsvAgent {
    pub fn new(
        conn: OsString,
        out: OsString
    ) -> Self {
        Self {
            conn: conn,
            out: out,
            my_data: Mydata::default(),
            data_store: HashMap::new(),
        }
    }

    pub fn csv_load(&mut self ) -> Result<(), Box<dyn Error>> {
    
        let file = File::open(self.conn.clone())?;
        let mut rdr = csv::ReaderBuilder::new()
            .from_reader(file);

        for res in rdr.records() {
            let record = res?;
            
            let id = &record[1];
            self.my_data.id = id.to_string();

            let date = &record[0];
            self.my_data.date = date.to_string();

            let prev_close = BigDecimal::from_f64(record[3].parse::<f64>()?);
            // if let Some(v) = prev_close {
            //     self.my_data.prev_close = v.clone();
            // }
            if let Some(v) = prev_close {
                self.data_store.insert(
                    id.clone(),
                    v
                );
            }
            
            let open = BigDecimal::from_f64(record[4].parse::<f64>()?);
            let close = BigDecimal::from_f64(record[8].parse::<f64>()?);
            let high = BigDecimal::from_f64(record[5].parse::<f64>()?);
            let low = BigDecimal::from_f64(record[6].parse::<f64>()?);
            let last = BigDecimal::from_f64(record[7].parse::<f64>()?);
            let volume = BigDecimal::from_f64(record[10].parse::<f64>()?);
            
            println!("{:?}", self.my_data);
             // println!(
            //     "id = {:?}, date = {:?}, prev_close = {:?}, open = {:?},
            //      close = {:?}, high = {:?}, low = {:?},
            //      last = {:?}, volume {:?}",
            //      id, date, prev_close,
            //      open, close, high, low,
            //      last, volume,
            // );

        }

        Ok(())
    }

    pub fn csv_load_hash(&self) -> Result<(), Box<dyn Error>> {
        //type Record = HashMap<String, String>;  

        let file = File::open(self.conn.clone())?;
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            let record: Hogedata = result?;
            println!("{:?}", record);
        }
        Ok(())
    }

    pub fn csv_header(&mut self) -> Result<(), Box<dyn Error>> {
        
        let file = File::open(self.conn.clone())?;
        let mut rdr = csv::Reader::from_reader(file);

        let headers = rdr.headers()?;
        println!("header: {:?}", headers);

        Ok(())
    }

    pub fn csv_write(&self) -> Result<(), Box<dyn Error>> {
        
        let file = File::create(self.out.clone())?;

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(file);

        wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
        wtr.serialize(("Davidsons Landing", "AK", None::<u64>, 65.2419444, -165.2716667))?;
        wtr.serialize(("Kenai", "AK", Some(7610), 60.5544444, -151.2583333))?;
        wtr.serialize(("Oakman", "AL", None::<u64>, 33.7133333, -87.3886111))?;
        
        wtr.flush()?;
        Ok(())
    }

    pub fn make_prev22_value() {
        //ここで値を作る？
        //もしくわcsvを吐き出す時にvalueを作る。
        //データ構造は　vec<Mydata(id, value)>
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