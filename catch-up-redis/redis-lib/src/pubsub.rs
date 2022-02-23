//
use anyhow::Result;
use redis::aio::PubSub;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::StreamExt;
use std::sync::Arc;
use log::*;

pub async fn setup_pubsub(path: &str) -> PubSub {
    let client = redis::Client::open(path).unwrap();

    let con = client.get_async_connection().await.unwrap();

    con.into_pubsub()
}

pub struct PubSubManager {
    conn: Arc<RwLock<PubSub>>,   
}

impl PubSubManager {
    pub async fn new() -> Self {
        Self {
            conn: Arc::new(RwLock::new(setup_pubsub("redis://localhost").await)),
        }
    }

    pub async fn subscribe_here(&self) -> Result<()> {

        //let sharecon = Arc::new(RwLock::new(self.conn));
        

        let c_lock1 = self.conn.clone();
        let task1 = tokio::spawn(async move{
            let mut pubsub1 = c_lock1.write().await;

            let _ = pubsub1.psubscribe("hoge*").await;

            let mut task1_stream = pubsub1.on_message();

            while let Some(v) = task1_stream.next().await {
                let res: String = v.get_payload().unwrap();
                info!("task1: {:?}", &res);
            }

            //ここで key: hoge* をpsubscribeする。
        });

        let c_lock2 = self.conn.clone();
        let task2 = tokio::spawn(async move {
            let mut pubsub2 = c_lock2.write().await;

            let _  = pubsub2.psubscribe("fugafuga*").await;

            let mut task2_stream = pubsub2.on_message();

            while let Some(v) = task2_stream.next().await {
                let res: String = v.get_payload().unwrap();
                info!("task2: {:?}", &res);
            }

            //ここで key: fugafuga* をpsubscribeする。
        });

        let _ = task1.await;
        let _ = task2.await;

        Ok(())
    }
}