pub mod redis_helper;
pub mod connection;
pub mod data;
pub mod pubsub;
pub mod utils;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
