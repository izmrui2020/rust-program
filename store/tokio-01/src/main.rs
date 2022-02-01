use anyhow::bail;
use anyhow::{Context, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::mpsc;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn handler(data: i32) -> i32 {

    data * 2
}

fn main() -> Result<()> {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let data = 12354;

    let handle = tokio::spawn(async move {
        let mut new  = vec![];
        let a = handler(data);

        new.push(a);
        let b = Some(123);

        

    });

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("m = {:?}", counter.lock().unwrap());

    Ok(())
}

