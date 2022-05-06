//
use anyhow::Result;
use crossbeam_channel::Receiver;
use super::under::Under;

pub struct Run {
    store: Under,
}

impl Run {
    pub fn new() -> Self {
        Run { 
            store: Under::new(),
        }
    }

    pub fn run(self: &'static mut Self) -> Result<()> {
        
        let task = tokio::spawn(async move {
            self.store.sort().await;
        });

        task.await;
        
        Ok(())
    }
}