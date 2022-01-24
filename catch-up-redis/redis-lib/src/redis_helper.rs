use anyhow::Result;
use redis::{aio::MultiplexedConnection, Client};
use super::data::{Data, MyData};

#[derive()]
pub struct RedisHelper {
    conn: MultiplexedConnection,
}

impl RedisHelper {
    pub async fn new() {
        
    }
    pub async fn setup_redis(&mut self, url: String) -> Result<()> {
        let cli = Client::open(url)?;

        self.conn = cli.get_multiplexed_async_connection().await?;

        Ok(())
    }

    pub async fn insert_hash(&self, data: &MyData) -> Result<()> {
        let mut con = self.conn.clone();
        let mut pipe = redis::pipe();

        pipe.cmd("HSET")
            .arg(data.id.clone())
            .arg(data)
            .ignore();
        
        let _: () = pipe.query_async(&mut con)
            .await?;
        
        Ok(())
    }
}