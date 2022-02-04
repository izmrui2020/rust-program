//
use anyhow::Result;
use std::{collections::HashMap, vec};

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

    let mut vec = vec![];
    let mut vec2 = vec![];

    for i in 1..100 {
        vec.push(i*12);
    }

    for i in 100..200 {
        vec2.push(i*10);
    }

    let res: Vec<_> = vec.into_iter().filter(|i| i & 2 == 0).collect();

    let res2 = vec.into_iter().filter(|i| {
        i == res.any(|a| a==i)
    }).collect();
    
    println!("Hello, world!");
}
