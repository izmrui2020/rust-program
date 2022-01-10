use structopt::StructOpt;

#[derive(StructOpt, Clone)]
struct Opt {
    name: String,
    #[structopt(short = "g", long)]
    greet: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    // if let Some(value) = opt.greet {
    //     println!("{}, {}", value, opt.name);
    // }
    if let None = opt.greet {
        println!("none");
    }
}
