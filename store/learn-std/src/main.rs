use std::{collections::HashMap};
use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use async_trait::async_trait;

#[async_trait]
pub trait Interface {
    fn create(&self);
    fn target_method(&mut self) -> Result<()>;
}
pub struct Data {
    nama: String,
    id: String,
}
pub trait Fuga {
    fn hogehoge(&self);
    fn update(&mut self) -> Result<()>;
    fn down(&mut self) -> Result<()>;
}
pub struct Hoge {
    store: HashMap<String, Box<dyn Fuga + Send + 'static>>,
}

#[async_trait]
impl Interface for Hoge {
    fn create(&self) {
        for i in (0..10) {
            let data = Sub::default();

            self.store.insert(
                i.to_string(),
                Box::new(data)
            );
        }
    }
    async fn target_method(&mut self) -> Result<()> {
        
        let (tx, rx) = mpsc::channel(32);
        
        self.store.iter_mut().for_each(|(_, target)| {
            let clone_tx1 = tx.clone();
            let clone_tx2 = tx.clone();

            let task1 = tokio::spawn(async move {
                let res = target.update();
                clone_tx1.send(res).await;
            });

            let task2 = tokio::spawn(async move {
                let res = target.down();
                clone_tx2.send(res).await;
            });

        });

        while let Some(v) = rx.recv().await {
            println!("hgoe:{:?}", &v);   
        };

        
        Ok(())
    }
}

#[derive(Default)]
pub struct Sub {
    hoge: String
}

impl Fuga for Sub {
    fn hogehoge(&self) {
        
    }
    fn update(&mut self) -> Result<()> {
        Ok(())
    }
    fn down(&mut self) -> Result<()>{
        Ok(())
    }
}

fn main() {
    
}