//
use communicate::conn_client::ConnClient;
use communicate::EchoRequest;
use serde::Deserialize;
use tonic::Streaming;

pub mod communicate {
    tonic::include_proto!("communicate");
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    host: String,
    port: u16,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 20000,
        }
    }
}
impl Config {
    fn load() -> Config {
        let mut cfg = config::Config::default();
        cfg.merge(config::Environment::with_prefix("client")).unwrap();
        cfg.try_into().unwrap_or_else(|_| Default::default())
    }
    
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::load();

    let mut client = ConnClient::connect(
        format!("http://{}:{}", cfg.host, cfg.port)).await?;

    let request = tonic::Request::new(EchoRequest {
        name: "Tonic".into(),
    });

    let response = client.echo(request).await?;

    println!("{:?}", response);
    Ok(())
}
