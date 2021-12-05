use std::fmt::Display;

pub trait Ranking {
    fn run();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Amount<T: Ranking + Display> {
    pub components: Vec<T>,
}

impl<T> Amount<T> where T: Ranking + Display {}

fn main() {}
