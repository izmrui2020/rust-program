//
use anyhow::Result;
use redis::aio::PubSub;

pub async fn setup_pubsub(path: &str) -> PubSub {
    let client = redis::Client::open(path).unwrap();

    let con = client.get_async_connection().await.unwrap();

    con.into_pubsub()
}