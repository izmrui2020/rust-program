use log::*;

use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod regi;
use regi::{Human, Lion, Seep, Tiger};

#[derive(EnumIter, Debug)]
pub enum Animal {
    Human(Human),
    Lion(Lion),
    Tiger(Seep),
    Seep(Tiger),
    //..etc
}

#[derive(Debug)]
pub struct StapFood {
    food_registor: HashMap<String, Vec<Animal>>,
}

fn main() {
    for i in Animal::iter() {
        println!("each animal iter(): {:?}", i);
        info!("each animal: {:?}", i);
    }
}
