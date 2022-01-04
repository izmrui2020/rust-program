use chrono::Duration;
use timer::Timer;
use async_trait::async_trait;
use std::time::{Instant};
use std::collections::HashMap;
use std::default::Default;
use std::sync::mpsc::channel;


mod ranking;
use ranking::Ranking1;

pub trait Ranking {
    fn create(&mut self);
    fn update(&mut self, data: String);
}

#[async_trait]
pub trait DefaultGen {
    async fn handle(&mut self);
    async fn sort_manager(&mut self);
}

pub struct RankingGenerator {
    ranking_store: HashMap<String, Box<dyn Ranking>>,
    sort_judgement: bool,
    time_keeper: Timer,
}

impl Default for RankingGenerator {
    fn default() -> RankingGenerator {
        RankingGenerator {
            ranking_store: HashMap::new(),
            sort_judgement: false,
            time_keeper: Timer::new(),
        }
    }
}

// impl DefaultGen for RankingGenerator {
//     fn handle(&mut self) {
//         self.ranking_store.insert(
//             "ranking1".to_string(),
//             Box::new(Ranking1::default())
//         );
//         self.ranking_store.iter_mut().for_each(|(_k, v)| {
//             v.create();
//         });
//     }
//     fn sort_manager(&mut self) {
       
//     }
// }

#[tokio::main]
async fn main() {
    let timer = Timer::new();
    
    let mut def = RankingGenerator::default();

    //def.handle().await;

}