use crate::Ranking;

#[derive(Default)]
pub struct Ranking1 {
    amount: String,
}


impl Ranking for Ranking1 {
    fn create(&mut self) {
        println!("this is create");
    }
    fn update(&mut self, data: String) {
        println!("this is update");
    }
}