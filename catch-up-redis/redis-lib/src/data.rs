use std::{collections::HashMap, io::Read};
use bigdecimal::BigDecimal;
use anyhow::{Result, bail, Context};
use redis::ToRedisArgs;

use super::redis_helper::RedisHelper;

#[derive(Debug, Default, Clone)]
pub struct MyData {
    pub id: String,
    pub value: BigDecimal,
    pub retio: i64,
}

pub struct Data {
    id: String,
    data_set: HashMap<String, MyData>,
    vec_store: Vec<MyData>,
}

impl Data {
    fn initalization(&mut self, id: String, data: MyData) -> Result<()> {
        self.id = id.clone();
        
        let in_data = self.data_set.insert(
            id.clone(),
            data.clone()
        );

        if let None = in_data {
            bail!("faile insert");
        }

        self.vec_store.push(data.clone());

        Ok(())
    }

    async fn add_cache(&self, id: String, conn: RedisHelper) -> Result<()> {

        let entry = self.vec_store
            .iter()
            .find(|k| k.id == id);

        if let Some(v) = entry {
            conn.insert_hash(v).await?;
        }

        Ok(())
    }
}

impl ToRedisArgs for MyData {
    fn to_redis_args(&self) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        self.write_redis_args(&mut out);
        out
    }

    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite 
    {
        let clone = self.clone();
        let bytes: &[u8]  = unsafe { any_as_u8_slice(&clone) };
        
        out.write_arg_fmt(serde_json::to_string(bytes).expect("Can't serialize Planet as string"))
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn hogehoge() {
        unimplemented!()
    }
    
}