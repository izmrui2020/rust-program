//
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
struct Help {
    num: f64,
    exe: bool,
}

#[derive(Debug, Clone, Default)]
pub  struct HogeHoge {
    hash: HashMap<String, Help>,
    que: Vec<u8>,
}

impl HogeHoge {
    async fn initial(&mut self, data: String) -> Result<()> {
        Ok(())
    }

    async fn handle(&mut self, data: String) -> Result<()> {

        
        Ok(())
    }

    async fn exe(&mut self, data: String) -> Result<()> {
        
        if self.que.len() != 0 {
            println!("hogehoge");
            self.que.clear();
        } 

        Ok(())
    }
    
}


fn main() {

    
    println!("Hello, world!");
}
