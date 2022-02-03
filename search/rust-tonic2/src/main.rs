//
use anyhow::Result;
use std::time::Instant;
use tonic::{transport::Server, Request, Response, Status};

use communicate::conn_server::{ConnServer, Conn};
use communicate::{EchoRequest, EchoReply};

pub mod communicate {
    tonic::include_proto!("communicate");
}

#[derive(Debug, Default)]
pub struct Handler {}

#[tonic::async_trait]
impl Conn for Handler {
    async fn echo(&self, req: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        println!("Got a request: {:?}", req);

        let replay = communicate::EchoReply {
            message: req.into_inner().name.clone(),
        };

        Ok(Response::new(replay))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:20000".parse()?;
    let conn = Handler::default();

    Server::builder()
        .add_service(ConnServer::new(conn))
        .serve(addr)
        .await?;

    Ok(())
}