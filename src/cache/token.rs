use r2d2_redis::redis::{Commands, self};
use r2d2_redis::{r2d2::PooledConnection, RedisConnectionManager};
use serde::{Deserialize, Serialize};
use serde_json;
use redis::RedisResult;
use super::error::{Error};
use base64::{encode, decode};

pub trait Identifier {
    fn id(&self) -> String;
}

fn token_encrypt(value: String) -> String{
    encode(value.into_bytes())
}

fn token_decrypt(value: String) -> String {
    String::from_utf8(decode(value).unwrap()).unwrap()
}

pub fn get_value<T>(value_key: String, conn: &mut redis::Connection) -> Option<T> where T: serde::de::DeserializeOwned {
    let r:RedisResult<String> = conn.get(value_key);
    match r {
        Ok(s) => {
            let v = serde_json::from_str(token_decrypt(s).as_ref());
            match v  {
                Ok(v) => v,
                Err(_) => None
            }
        },
        Err(err) => None
    }
}

pub fn get_value2<T>(value_key: &str, conn: &mut redis::Connection) -> Option<T> where T: for<'de> Deserialize<'de> {
    let r:RedisResult<String> = conn.get(value_key);
    match r {
        Ok(s) => {
            let v = serde_json::from_str(s.as_str());
            match v  {
                Ok(v) => v,
                Err(_) => None
            }
        },
        Err(err) => None
    }
}

pub fn store_token<T>(token: T, conn: &mut redis::Connection) -> std::result::Result<String, Error>
where
    T: Identifier + Serialize,
{
    let v = token_encrypt(serde_json::to_string(&token).unwrap());
    let v_clone = v.clone();
    let ex: usize = 60 * 60 * 24 * 7;
    let res: RedisResult<String> = conn.set_ex(token.id(), v, ex);
    match res {
        Ok(_) => Ok(v_clone),
        Err(e) => Err(Error::Other(e))
    }
}
