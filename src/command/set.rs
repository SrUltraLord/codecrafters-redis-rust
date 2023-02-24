use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tokio::sync::Mutex;

use crate::marshall::{bulk_string_marshaller, string_marshaller};
use crate::sync::redis_value::RedisValue;

pub const NAME: &str = "SET";
const REQUIRED_ARGS: usize = 2;
const PX_ARGS: usize = 2;

pub async fn handle(
    args: &Vec<String>,
    map: Arc<Mutex<HashMap<String, RedisValue>>>,
) -> Result<String, String> {
    let total_args = args.len();
    if total_args < REQUIRED_ARGS {
        return Err("ERR wrong number of arguments for 'set' command.".to_string());
    }

    if total_args != REQUIRED_ARGS && total_args != REQUIRED_ARGS + PX_ARGS {
        return Err("Err syntax error".to_string());
    }

    let mut map = map.lock().await;

    let (k, v) = (args[0].to_string(), args[1].to_string());

    let key = bulk_string_marshaller::marshall(k);
    let value = bulk_string_marshaller::marshall(v);

    let mut expire_at: Option<SystemTime> = None;

    if total_args == REQUIRED_ARGS + PX_ARGS {
        if args[2].to_lowercase() != "px".to_string() {
            return Err("Err syntax error".to_string());
        }

        let expire_time_ms = args[3].parse::<u64>();

        if let Err(_) = expire_time_ms {
            return Err("ERR value is not an integer or out of range".to_string());
        }

        let now = SystemTime::now();
        let duration_ms = Duration::from_millis(expire_time_ms.unwrap());

        expire_at = Some(now + duration_ms);
    }

    map.insert(key, RedisValue { value, expire_at });

    Ok(string_marshaller::marshall("OK".to_string()))
}
