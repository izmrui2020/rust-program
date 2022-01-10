use tokio::io;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::new::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    let (mut rd, mut wr) = io::split(socket);

    let write_task = tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        Ok::<_, io::Error>(())
    })

    let mut buf = vec![0; 128];
    
    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}
