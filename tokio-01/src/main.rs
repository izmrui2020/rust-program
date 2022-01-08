use anyhow::{Context, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::mpsc;

mod hoge;
use hoge::Hoge;

#[tokio::main]
async fn main() -> Result<()> {
    Hoge::iter().for_each(|v| {});

    Ok(())
}
