use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::marshall::bulk_string_marshaller;

pub const NAME: &str = "GET";
const REQUIRED_ARGS: usize = 1;

pub async fn handle(
    args: &Vec<String>,
    map: Arc<Mutex<HashMap<String, String>>>,
) -> Result<String, String> {
    let map = map.lock().await;

    if args.len() != REQUIRED_ARGS {
        return Err("ERR wrong number of arguments for 'get' command.".to_string());
    }

    let key = args[0].to_string();

    let serialized_key = bulk_string_marshaller::marshall(key);
    let result = match map.get(&serialized_key) {
        Some(value) => value.to_owned(),
        None => bulk_string_marshaller::NIL.to_string(),
    };

    Ok(result)
}
