use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::marshall::{bulk_string_marshaller, error_marshaller, string_marshaller};

pub const NAME: &str = "SET";
const REQUIRED_ARGS: usize = 2;

pub async fn handle(args: &Vec<String>, map: Arc<Mutex<HashMap<String, String>>>) -> String {
    let mut map = map.lock().await;

    if args.len() != REQUIRED_ARGS {
        return error_marshaller::marshall(
            "ERR wrong number of arguments for 'set' command.".to_string(),
        );
    }

    let key = args[0].to_string();
    let value = args[1].to_string();

    let serialized_key = bulk_string_marshaller::marshall(key);
    let serialized_value = bulk_string_marshaller::marshall(value);

    map.insert(serialized_key, serialized_value);

    string_marshaller::marshall("OK".to_string())
}
