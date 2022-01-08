use anyhow::{Context, Result};
use rand::Rng;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::mpsc;

#[derive(EnumIter, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hoge {
    A,
    B,
    C,
    D,
    E,
    F,
}

pub struct Store {
    store: HashMap<i64, i64>,
}

impl Store {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    fn create(&mut self) -> Result<()> {
        let rnd = rand::thread_rng();
        for k in Hoge::iter() {
            let h: i64 = rng.gen();
            self.store.insert(k(), h)
        }
        Ok(())
    }
}
