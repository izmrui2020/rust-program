//
use anyhow::Result;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

mod sub;


#[tokio::main]
async fn main() -> Result<()> {
    let test = BigInt::new(num_bigint::Sign::Minus, vec![12, 784]);
    let test2 = BigInt::new(num_bigint::Sign::Plus, vec![1242342, 784, 192, 79453, 7493287, 924387]);

    let demo1 = BigDecimal::from(test);
    let demo2 = BigDecimal::from(test2);

    let percent = demo1 / demo2;

    let res = percent.round(2);

    println!("{}", res);


    Ok(())
}
