//
use anyhow::Result;
use crossbeam_channel::Receiver;
use super::under::Under;

struct Run {
    store: Under,
}

impl Run {
    pub fn new() -> Self {
        Run { 
            store: Under::new(),
        }
    }

    pub async fn run(self: &'static mut Self) -> Result<()> {
        
        let task = tokio::spawn(async move {
            self.store.sort().await;
        });

        task.await;
        
        Ok(())
    }
}