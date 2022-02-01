//
use structopt::{clap::{self, arg_enum}, StructOpt};
use std::path::PathBuf;
use hyper::http::{Request, Response};

#[derive(StructOpt, Debug)]
#[structopt(name = "example")]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))] //git のリビジョン
#[structopt(setting(clap::AppSettings::ColoredHelp))] //put color
pub struct Opt {
    #[structopt(short = "c", long = "csv_url", parse(from_os_str))]
    pub csv_path: PathBuf,
    #[structopt(short = "u", long = "url")]
    pub url: String,
    #[structopt(short = "h", long = "cmd")]
    pub cmd: Cmd,
}

arg_enum! {
    #[derive(Debug)]
    pub enum Cmd {
        A,
        B,
        C,
    }
}

async fn create_hyper_conn(url: String) -> anyhow::Result<hyper::client::conn::SendRequest<hyper::Body>> {
    let stream = tokio::net::TcpStream::connect(url).await?;
    stream.set_nodelay(true)?;
    stream.set_linger(std::time::Duration::from_secs(1).into())?; //https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#method.set_linger
    let (send, conn) = hyper::client::conn::handshake(stream).await?;
    tokio::spawn(conn);

    Ok(send)
}


fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut request = Request::builder()
        .uri("https://www.rust-lang.org/")
        .header("User-Agent", "my-awesome-agent/1.0");

    if needs_awesome_header() {
        request = request.header("Awesome", "yes");
    }

    let response = send(request.body(()).unwrap());

    fn send(req: Request<()>) -> Response<()> {
        // ...
    }
}
