use timer::Timer;
use chrono::Duration;
use async_timer::Interval;
use std::sync::mpsc::channel;
use tokio::sync::oneshot;

async fn job() {
    println!("hello");
}

async fn do_a_while() {
    let mut interval = Interval::platform_new(core::time::Duration::from_secs(3));

    loop {
        interval.as_mut().await;
        job().await;
    }
}

#[tokio::main]
async fn main() {

    println!("this is main side");

    let handle = tokio::spawn(async move {
        do_a_while().await;
    });
    

    println!("this is back sidee");

    

    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("I am final");

    handle.await;
}
