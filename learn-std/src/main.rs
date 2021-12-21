fn main() {
    let num = vec![5];
    let arg = vec![6];

    println!("num: {:p}", &num);
    println!("arg: {:p}", &arg);

    let c2 = |arg| {
        println!("num: {:p}", &num);
        println!("arg: {:p}", arg);
    };
    c2(&arg);
    println!("num: {:p}", &num);
    println!("arg: {:p}", &arg);
}
