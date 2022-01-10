use bigdecimal::BigDecimal;
use std::str::FromStr;

fn main() {
    
    let input = "0.8";
    let dec = BigDecimal::from_str(&input).unwrap();
    let float = f32::from_str(&input).unwrap();

    println!("dec: {}, float: {}", dec, float);


}
