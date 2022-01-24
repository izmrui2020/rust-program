//
use redis_lib::data::Data;
use redis_lib::redis_helper::RedisHelper;
use redis::aio::MultiplexedConnection;

fn main() {
    
    let conn = MultiplexedConnection::new();
}
