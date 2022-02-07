//
use anyhow::Result;
use tokio_cron_scheduler::{JobScheduler, Job};
use std::{collections::HashMap, vec};
use tokio::sync::mpsc;

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
    fn new() -> Self {
        Self {
            hash: HashMap::new(),
            que: Vec::new(),
        }
    }

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


#[tokio::main]
async fn main() -> Result<()> {

    let instance = HogeHoge::new();

    let (tx, rx) = mpsc::channel(32);
    let mut tx2 = tx.clone();

    let mut sched = JobScheduler::new();
    // 毎日12時15分
    let job = Job::new("15 12 * * *", |_uuid, _lock| {
            tx2.send("hoge");
    });
    sched.add(job);

    tokio::spawn(sched.start());

    let a = tokio::spawn(async move {
        sched.start()
    });

    tokio::join!(a);
    
    
    
    
    Ok(())
}
