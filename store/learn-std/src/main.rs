use std::{collections::HashMap};
use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use std::sync::{Arc,Mutex};
use async_trait::async_trait;

#[async_trait]
pub trait Interface {
    fn create(&mut self);
    async fn target_method(&'static mut self) -> Result<()>;
}
pub struct Data {
    nama: String,
    id: String,
}
#[async_trait]
pub trait Fuga {
    fn hogehoge(&self);
    fn update(&mut self) -> Result<()>;
    fn down(&mut self) -> Result<()>;
}

pub struct Hoge<T> 
where
    T: Fuga + Send + Default + 'static,
{
    store: HashMap<String, Box<T>>,
}

async fn target_process_async<T>(
    target: &'static mut Box<T>,
    tx: Sender<Result<()>>,
) -> Result<()>
where
    T: Fuga + Send + Default + 'static,
{
    let res = target.update();
    tx.send(res).await?;

    let res = target.down();
    tx.send(res).await?;
    Ok(())
}

#[async_trait]
impl<T> Interface for Hoge<T> 
where
    T: Fuga + Send + Default + 'static
{
    fn create(&mut self) {
        for i in (0..10) {
            let data = T::default();

            self.store.insert(
                i.to_string(),
                Box::new(data)
            );
        }
    }
    async fn target_method(&'static mut self) -> Result<()> {
        
        let (tx, mut rx) = mpsc::channel(32);
        let mut handles = vec![];

        self.store.iter_mut().for_each(|(key, target)| {
            let clone_tx1 = tx.clone();
                let task1 = tokio::spawn(async move {
                    let res = target.update();
                    match clone_tx1.send(res).await {
                        Err(e) => anyhow::bail!(e),
                        _ => {}
                    }

                    let res = target.down();
                    match clone_tx1.send(res).await {
                        Err(err) => anyhow::bail!(err),
                        _ => {}
                    }
                    Ok(())

            
            });
            handles.push(task1);

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

#[async_trait]
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