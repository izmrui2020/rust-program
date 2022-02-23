use std::f32::consts::E;


#[derive(Clone, Debug)]
struct Hoge {
    hoge: String,
}

fn main() {

    let mut hoge = Hoge {hoge: "hogehgoe".to_string()};

    let mut a = 10;
    let ref_a = &a;
    let ref_b = &mut a;
    let ref_c = &mut hoge;

    ref_c.hoge = "fugafuga".to_string();
    hoge.hoge = ref_c.hoge;

    println!("{:?}", hoge);
}
