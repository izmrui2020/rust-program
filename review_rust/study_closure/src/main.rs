
//
use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T>
where
    T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumCount};
use tokio_stream::StreamExt;
#[derive(EnumIter, Clone, Debug, PartialEq, Eq, Hash)]
enum Item {
    AAA,
    BBB,
    CCC,
    DDD,
    EEE,
}

async fn hogehoge() {
    let mut stream = tokio_stream::iter(Item::iter());

    while let Some(v) = stream.next().await {
        println!("hgeohoge: {:?}", v);
    }
}

#[tokio::main]
async fn main() {
    
    for _i in 1..10000 {
        hogehoge().await;
    }
    
}
