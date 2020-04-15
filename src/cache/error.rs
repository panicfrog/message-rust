use redis::RedisError;

#[derive(Debug)]
pub enum Error {
    Other(RedisError),
}
