//
use anyhow::Result;
use std::{collections::HashMap, vec};
use std::string::ToString;
use std::ops::{Deref, DerefMut};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default, Clone)]
pub struct Data {
    id: String,
    name: String,
    value: i32,
}

#[derive(EnumIter, ToString, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Index {
    PatternA,
    PatternB,
    PatternC,
    PatternD,
}

#[derive(Debug, Default, Clone)]
pub struct Helper {
    up: Vec<Data>,
    down: Vec<Data>,
}
impl Helper {
    pub fn new() -> Self {
        Self {
            up: vec![],
            down: vec![],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Target {
    store: HashMap<Index, Helper>,
}

impl Deref for Target {
    type Target = HashMap<Index, Helper>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl DerefMut for Target {
    fn deref_mut(&mut self) -> &mut HashMap<Index, Helper> {
        &mut self.store
    }
}

impl Target {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        Index::iter().for_each(|i| {
            self.store.insert(
                i,
                Helper::new(),
            );
        });

        Ok(())
    }

    pub async fn update<'a>(&'a mut self) -> Result<()> {

        // let multi_ref = Arc::new(RwLock::new(&self.store));
        // let mut executor = vec![];

        // for index in Index::iter() {
        //     let key= format!("{}:id", index.to_string());

        //     let clone_ref = Arc::clone(&multi_ref);
        //     let mut wt = clone_ref.write().await;
        //     let mut data_store = wt.deref_mut().clone();

        //     if let Some(data_vec) = data_store.get_mut(&index) {
        //         data_vec.up.sort_by(|a, b| b.value.cmp(&a.value));

        //         data_vec.down.sort_by(|a, b| a.value.cmp(&b.value));

        //         let up_task = insert_redis(&data_vec.up, &key);
        //         let down_taks = insert_redis(&data_vec.down, &key);

        //         executor.push(up_task);
        //         executor.push(down_taks);
        //     }
        // }

        // futures::future::join_all(executor).await;
        Ok(())
    }
}


pub async fn insert_redis(data: &[Data], key: &str) -> Result<()> {

    Ok(())
}