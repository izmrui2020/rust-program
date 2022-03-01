//
use anyhow::Result;
use strum_macros::{EnumIter};
use strum::IntoEnumIterator;
use std::{collections::HashMap};

#[derive(Debug, Default)]
struct ItemA {
    st: HashMap<String, String>,
}

impl ItemA {
    async fn do_it(&mut self) -> Result<()> {
        self.st.insert(
            "hogehoge".to_string(),
            "fugafuga".to_string(),
        );
        Ok(())
    }
}

#[derive(Debug, Default)]
struct ItemB {
    st: HashMap<String, String>,
}

impl ItemB {
    async fn do_it(&mut self) -> Result<()> {
        self.st.insert(
            "hogehoge".to_string(),
            "fugafuga".to_string(),
        );
        Ok(())
    }
}


#[derive(EnumIter, Hash, PartialEq, Eq, Clone, Debug)]
pub enum Index {
    IndexA,
    IndexB
}

impl Index {
    fn return_indicator(&self) -> Indicator {
        match self {
            Index::IndexA => Indicator::ItemA(ItemA::default()),
            Index::IndexB => Indicator::ItemB(ItemB::default()),
        }       
    }
}

#[derive(Debug)]
pub enum Indicator {
    ItemA(ItemA),
    ItemB(ItemB)
}

impl Indicator {
    async fn puress(&mut self) -> Result<()> {
        
        Ok(())
    }

    async fn do_it_each(&mut self) -> Result<()> {
        match self {
            Indicator::ItemA(ref mut itema) => {
                itema.do_it();
            },
            
        }
        
        Ok(())
    }
}

#[derive(Debug)]
struct Store {
    store: HashMap<Index, Indicator>,
}

impl Store {
    async fn create_index(&mut self) -> Result<()> {
        for i in Index::iter() {
            self.store.entry(i.clone())
                .or_insert_with(|| i.return_indicator())
                .puress();
        }

        println!("create_index{:?}", self.store);
        
        Ok(())
    }
}