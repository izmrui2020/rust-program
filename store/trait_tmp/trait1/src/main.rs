use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod mod_traits {
    pub trait Out {
        fn print_twice(&self);
        fn print_add_doller(&self);
    }
}
use mod_traits::Out;

#[derive(Debug)]
pub struct Human {
    hogehoge: i64,
}
#[derive(Debug)]
pub struct Lion {
    hogehoge: String,
}

#[derive(Debug)]
pub struct StaplFood {
    // food_store: HashMap<Animal, Vec<Animal>>,
}

impl StaplFood {
    fn yen_mark(&self) {
        println!("¥ {:?}", self.food_store);
    }
}

impl Out for StaplFood {
    fn print_twice(&self) {
        println!("{:?}, {:?}", self.food_store, self.food_store);
    }
    fn print_add_doller(&self) {
        println!("$ {:?}", self.food_store);
    }
}

fn main() {
    let input = Expression::SomeValue.iter();

    // let output = if let Some(val) = input {
    //     Some(func(val))
    // } else {
    //     None
    // };
}
