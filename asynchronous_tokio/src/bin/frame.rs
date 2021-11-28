use bytes::Bytes;
use mini_redis::{Frame, Result};
use tokio::net::TcpStream;

// enum Frame {
//     Simple(String),
//     Error(String),
//     Integer(u64),
//     Bulk(Bytes),
//     Null,
//     Array(Vec<Frame>),
// }
struct Connection {
    stream: TcpStream,
    // ... 他のフィールドを書く
}

impl Connection {
    /// コネクションからフレームを読み取る
    ///
    /// EOF に到達したら `None` を返す
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        // ここに実装を書く
    }

    /// コネクションにフレームを書き込む
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        // ここに実装を書く
    }
}
