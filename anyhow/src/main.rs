// use anyhow::*;
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> Result<()> {
//     File::open("foo.txt").await?;
//     Ok(())

// }

// use anyhow::*;
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> Result<()> {
//     File::open("foo.txt").await?;
//     Ok(())

// }


// use std::fmt::Display;
// use std::error::Error;
// use anyhow::{Result, Context, anyhow};

// #[derive(Debug)]
// enum MyError {
//     HogeError(u8),
//     FooError(String),
// }

// impl Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         use self::MyError::*;
//         match self {
//             HogeError(i) => write!(f, "HogeError: {}", i),
//             FooError(s) => write!(f, "FooError: {}", s),
//         }
//     }
// }

// impl Error for MyError {}
    

// fn hoge(filename: &str) -> anyhow::Result<()> {
//     //let f = std::fs::File::open(filename).with_context(|| format!("faile to open filr: {}", filename))?;                 // std::io::Errorを返す
    
//     std::fs::File::open(filename).map_err(|e| anyhow!(e))?;
    
//     // ...
//     Err(MyError::FooError("foo".to_string()))?;    // MyErrorを返す
//     // ...
//     Ok(())
// }

use tokio::fs::File;
//use tokio::sync::mpsc;
use async_channel::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let (tx, rx) = bounded(32).await;

    
    let handle = tokio::spawn( async move {
        File::open("hogehoge").await
    });
    
    //hoge("hogehoge.tx").await?;

    println!("Hello, world!");

    handle.await??;

    Ok(())
}
