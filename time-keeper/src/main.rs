use std::time::Instant;
use std::collections::HashMap;
use std::default::Default;

mod ranking;
use ranking::Ranking1;

pub trait Ranking {
    fn create(&mut self);
    fn update(&mut self, data: String);
}

pub trait DefaultGen {
    fn handle(&mut self);
    fn sort_manager(&mut self);
}

pub struct RankingGenerator {
    ranking_store: HashMap<String, Box<dyn Ranking>>,
    sort_judgement: bool,
    time_keeper: String,
}

impl Default for RankingGenerator {
    fn default() -> RankingGenerator {
        RankingGenerator {
            ranking_store: HashMap::new(),
            sort_judgement: false,
            time_keeper: "hogehoge".to_string(),
        }
    }
}

impl DefaultGen for RankingGenerator {
    fn handle(&mut self) {
        self.ranking_store.insert(
            "ranking1".to_string(),
            Box::new(Ranking1::default())
        );
        self.ranking_store.iter_mut().for_each(|(_k, v)| {
            v.create();
        });
    }
    fn sort_manager(&mut self) {
        
    }
}


fn main() {
    
    let mut def = RankingGenerator::default();

    def.handle();

}