use super::error::Error;
use base64::{decode, encode};
use r2d2_redis::redis::{self, Commands, RedisResult};
use serde::{Serialize};
use serde_json;

pub trait Identifier {
    fn id(&self) -> String;
}

pub fn verification_value<T>(value: String, conn: &mut redis::Connection) -> Result<String, ()>
where
    T: serde::de::DeserializeOwned,
    T: Identifier,
{
    match get_id_from_value::<T>(value) {
        Err(_) => Err(()),
        Ok(id) => {
            if let Some(r) = get_value::<T>(id, conn) {
                Ok(Identifier::id(&r))
            } else {
                Err(())
            }
        }
    }
}

fn get_id_from_value<T>(value: String) -> Result<String, ()>
where
    T: serde::de::DeserializeOwned,
    T: Identifier,
{
    let d_token = token_decrypt(value);
    match d_token {
        Ok(d_token) => {
            let u = serde_json::from_str::<T>(d_token.as_ref());
            match u {
                Ok(u) => Ok(Identifier::id(&u)),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

fn token_encrypt(value: String) -> String {
    encode(value.into_bytes())
}

fn token_decrypt(value: String) -> Result<String, ()> {
    match decode(value) {
        Ok(v) => String::from_utf8(v).map_err(|_| ()),
        Err(_) => Err(()),
    }
}

pub fn get_value<T>(value_key: String, conn: &mut redis::Connection) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let r: RedisResult<String> = conn.get(value_key);
    match r {
        Ok(s) => {
            let s = token_decrypt(s);
            match s {
                Ok(s) => {
                    let v = serde_json::from_str(s.as_ref());
                    match v {
                        Ok(v) => v,
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

// pub fn get_value2<T>(value_key: &str, conn: &mut redis::Connection) -> Option<T>
// where
//     T: for<'de> Deserialize<'de>,
// {
//     let r: RedisResult<String> = conn.get(value_key);
//     match r {
//         Ok(s) => {
//             let v = serde_json::from_str(s.as_str());
//             match v {
//                 Ok(v) => v,
//                 Err(_) => None,
//             }
//         }
//         Err(err) => None,
//     }
// }

pub fn store_value<T>(value: T, conn: &mut redis::Connection) -> std::result::Result<String, Error>
where
    T: Identifier + Serialize,
{
    let v = token_encrypt(serde_json::to_string(&value).unwrap());
    let v_clone = v.clone();
    let ex: usize = 60 * 60 * 24 * 7;
    let res: RedisResult<String> = conn.set_ex(value.id(), v, ex);
    match res {
        Ok(_) => Ok(v_clone),
        Err(e) => Err(Error::Other(e)),
    }
}

#[test]
fn text_encode_decode() {
    let v = "{\"user_name\":\"yeyongping1\",\"passwd\":\"123456abc\"}".to_owned();
    let ecoded = token_encrypt(v.clone());
    let decoded = token_decrypt(ecoded).unwrap();
    assert_eq!(v, decoded);
}
