

enum MyError {
    IoError(std::io::Error),
    MyError(String),
    OtherError(Box<dyn std::error::Error>),
}

fn main() {

    let s  = std::fs::read_to_string("myfile").map_err(MyError::IoError);
    let v = vec![1usize, 2, 3];
    let v = v.into_iter().map(Some).collect::<Vec<_>>();

    if let Ok(s1) = std::env::var("hgeoghe") {
        if let Ok(s2) = std::env::var("fugafuga") {

        }
    }

    if let (Ok(s1), Ok(s2)) = (std::env::var("hogeho"), std::env::var("fugafuga")) {
        println!("hogehoge: {:?}, fugafuga: {:?}", s1, s2);
    }

    match (std::env::var("hogehoge"), std::env::var("fugafuga")) {
        (Ok(s), Err(_)) | (_, Ok(s)) => println!("{:?}", s);
    }

}