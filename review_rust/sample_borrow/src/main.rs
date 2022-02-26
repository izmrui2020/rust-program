use std::future::Future;
//
use std::io::{self, Write};
use anyhow::Result;
use std::pin::Pin;

fn main() {
    let rect = (30, 50);
    let width = 30;
    let heigh = 40;
    println!("hogehoge{}",area(rect));
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn recursion<'a, 'b>(some_ref: &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + 'b + Send>>
where
    'a: 'b,
{
    Box::pin(async move {
        let result = recursion(some_ref).await?;
        Ok(result.to_string())
    })
}