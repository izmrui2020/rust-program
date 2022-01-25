//
use rand::prelude::*;
use rand::Rng;
use bigdecimal::BigDecimal;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Data {
    pub id: String,
    pub value: BigDecimal,
    pub diff: Diff,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct Diff {
    value: u64,
    com: i64
}

impl Diff {
    fn new(
        value: u64,
        com: i64
    ) -> Self {
        Self {
            value,
            com
        }
    }
    
}

impl Data {
    fn new(
        id: String,
        value: BigDecimal,
        diff: f64
    ) -> Self {
        Self {
            id,
            value,
            diff: Diff::default()
        }
    }
}

struct Mdata {
    hash: HashMap<String, Data>,
    vec: Vec<Data>
}

impl Mdata {
    fn new() -> Self {
        Self {
            hash: HashMap::new(),
            vec: Vec::new(),
        }
    }
    fn set_value(&mut self) {
        
        let mut store: Vec<Data>;
        for i in 0..29 {
            let mut rnd = rand::thread_rng();
            let v = rnd.gen_range(0..1000);
            let f: f64 = rnd.gen_range(1..100) as f64;

            let t = Data::new(
                i.to_string(),
                BigDecimal::from(v),
                f
            );

            self.vec.push(t.clone());
            self.hash.insert(
                i.to_string(),
                t
            );
        }
    }
    
}

fn run() -> Vec<Data> {
    let mut instance = Mdata::new();

    instance.set_value();

    instance.vec

}


#[cfg(test)]
mod tests {
    use std::vec;
    use std::io::Write;
    use super::*;

    #[test]
    //#[ignroe]
    fn test_sort() {
        let mut i = Mdata::new();
        i.set_value();

        let mut v1 = i.vec;
        v1.sort_by(|a, b| a.value.cmp(&b.value));
        let mut an1 = vec![];
        v1.iter().for_each(|v| {
            an1.push(v.value.clone())
        });

        let mut v2 = run();
        v2.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        let mut an2 = vec![];
        v2.iter().for_each(|v| {
            an2.push(v);
        });

        writeln!(&mut std::io::stderr(), "{:?}", an1);
        
        run();
    }
}