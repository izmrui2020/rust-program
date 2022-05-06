
use anyhow::Result;
use demo1::channel::Run;

#[tokio::main]
async fn main() -> Result<()> {
    let mut hoge = Run::new();

    hoge.run().await;

    Ok(())
}
