//
use std::collections::HashMap;

struct HumanA;
struct HumanB;
struct HumanC;

enum Indicator {
    HumanA(HumanA),
    HumanB(HumanB),
    HumanC(HumanC),
}

#[derive()]
struct Hoge {
    store: HashMap<String, Indicator>,
}

impl Hoge {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

fn call_with_42<F>(f: F) 
where F: FnOnce(i32) -> i32 
{
    println!("f(42) = {}", f(42));
}

fn my_fn<T: std::fmt::Debug>() -> {
    
}

fn main() {

    call_with_42(|x|  2 * x);

    
}
