use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::Mutex;

use crate::marshall::bulk_string_marshaller;
use crate::sync::redis_value::RedisValue;

pub const NAME: &str = "GET";
const REQUIRED_ARGS: usize = 1;

pub async fn handle(
    args: &Vec<String>,
    map: Arc<Mutex<HashMap<String, RedisValue>>>,
) -> Result<String, String> {
    if args.len() != REQUIRED_ARGS {
        return Err("ERR wrong number of arguments for 'get' command.".to_string());
    }

    let mut map = map.lock().await;

    let key = args[0].to_string();

    let serialized_key = bulk_string_marshaller::marshall(key);

    let result = map.get(&serialized_key);
    if let None = result {
        return Ok(bulk_string_marshaller::NIL.to_string());
    }

    let expire_at = result.unwrap().expire_at.to_owned();

    if let Some(expire_at) = expire_at {
        let now = SystemTime::now();

        if now > expire_at {
            map.remove(&serialized_key);
            return Ok(bulk_string_marshaller::NIL.to_string());
        }
    }

    Ok(result.unwrap().value.to_string())
}
