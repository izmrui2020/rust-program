//
use rand::prelude::*;
use rand::Rng;
use bigdecimal::BigDecimal;
use std::cmp::Ordering;
use std::vec::Vec;
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

    

    // fn my_sort_algorithm<F>(&mut self, mut compare: F)
    // where
    //     F: FnMut(BigDecimal, BigDecimal) -> Ordering,
    // {
    //     self.merge_sort(|a, b| compare(a, b) == Ordering::Less);
    // }
}

fn my_merge_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}

pub fn merge(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut L1 = arr.clone();
    let mut R1 = arr.clone();
    let L = &L1[left..mid];
    let R = &R1[mid..right];
    /* Merge the temp arrays back into arr[l..r]*/
    let mut i = 0; // Initial index of first subarray
    let mut j = 0; // Initial index of second subarray
    let mut k = left; // Initial index of merged subarray
    while i < n1 && j < n2 {
        if L[i] < R[j] {
            arr[k] = L[i];
            i = i + 1;
        } else {
            arr[k] = R[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        arr[k] = L[i];
        i = i + 1;
        k = k + 1;
    }
    /* Copy the remaining elements of R[], if there
    are any */
    while j < n2 {
        arr[k] = R[j];
        j = j + 1;
        k = k + 1;
    }
    arr
}

pub fn calc_random_walk_steps(r:i64) -> i64 {
    let directions = [[-1,0],[1,0],[0,-1],[0,1]];
    let mut position = [0i64,0];
    let mut steps = 0;
    while (position[0].abs() + position[1].abs()) < r {
        let dir_index = rand::random::<usize>()%4;
        position[0]+=directions[dir_index][0];
        position[1]+=directions[dir_index][1];
        steps+=1;
    }    
    steps
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