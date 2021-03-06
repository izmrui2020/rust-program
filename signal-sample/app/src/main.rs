//
use anyhow::Result;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::consts::signal::*;
use std::sync::Condvar;

#[derive(Debug)]
enum Command {
    SendA,
    SendB,
    SendC
}

#[tokio::main]
async fn main() -> Result<()> {

    let (tx, mut rx) = mpsc::channel(10);

    let cond = Arc::new(Condvar::new());

    let hook = Arc::new(AtomicBool::new(false));
    let _ = signal_hook::flag::register(SIGHUP, Arc::clone(&hook));
    
    let task1 = tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_micros(100)).await;

            if hook.load(Ordering::Relaxed) {
                tx.send(Command::SendA).await.unwrap();
                hook.store(false, Ordering::Relaxed);
            }
        }
    });

    let receive_task = tokio::spawn(async move {
        while let Some(send) = rx.recv().await {
            match send {
                Command::SendA => println!("get SendA"),
                Command::SendB => println!("get SendB"),
                Command::SendC => println!("get SendC"),
            }
        }
    });

    let (task1,receive_task ) = tokio::join!(task1, receive_task);
    task1?;
    receive_task?;

    Ok(())
}
