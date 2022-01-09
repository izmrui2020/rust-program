// use anyhow::{Context, Result};
// use rand::Rng;
// use strum_macros::EnumIter;
// use tokio::sync::mpsc;
// use strum::{IntoEnumIterator};
// use std::collections::HashMap;

// #[derive(EnumIter,Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Hoge {
//     A,
//     B,
//     C,
//     D,
//     E,
//     F
// }


// pub struct Store {
//     store: HashMap<String, String>,
// }

// impl Store {
//     fn new() -> Self {
//         Self {
//             store: HashMap::new(),
//         }
//     }
//     fn create(&mut self) -> Result<()> {
//         let rnd = rand::thread_rng();
//         for k in Hoge::iter() {
//             let h: i = rng.gen();
//             self.store.insert(
//                 k.to_string(),
//                 ""
//             )
//         };
//         Ok(())
//     }
// }