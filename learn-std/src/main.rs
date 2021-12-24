use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
trait Hoge {
    fn hoge();
}

#[derive(Default)]
struct A {
    hoge: String
}
impl Hoge for A {fn hoge(){}}

#[derive(Default)]
struct B {
    hoge: String,
}
impl Hoge for B {fn hoge(){}}

#[derive(Default)]
struct C {
    hoge: String,
}
impl Hoge for C {fn hoge(){}}


#[derive(EnumIter)]
enum StructTypes<T: Hoge + Default> {
    A(T),
    B(T),
    C(T),
}

impl<T> StructTypes<T>
where
    T: Hoge + Default
{
    fn new(&self) {
        match self {
            A(_) => {self.A.hoge()},
            B(_) => {self.hoge()},
            C(_) => {self.hoge()},
        }
    }
}

struct EnumMaster<T: Hoge + Default> {
    store: HashMap<String, StructTypes<T>>
}

impl<T> EnumMaster<T>
where
    T: Hoge + Default
{
    
    fn hoge() {
        for i in StructTypes::<T>::iter() {
            
        }
    }
}





fn main() {
    println!("Hello, world!");
}