use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::command::{echo, get, ping, set};
use crate::marshall::{array_marshaller, error_marshaller};

pub async fn handle_command_from_buffer(
    buffer: &[u8],
    map: Arc<Mutex<HashMap<String, String>>>,
) -> String {
    let command_args = array_marshaller::unmarshall_from_buffer(buffer);
    let (command, args) = parse_array_as_command(command_args.as_slice());

    let result: Result<String, String> = match command.as_str() {
        ping::NAME => ping::handle(&args),
        echo::NAME => echo::handle(&args),
        set::NAME => set::handle(&args, map).await,
        get::NAME => get::handle(&args, map).await,
        _ => Err(format!("ERR unkown command '{}'", command.to_lowercase())),
    };

    match result {
        Ok(v) => v,
        Err(e) => error_marshaller::marshall(e),
    }
}

fn parse_array_as_command(command_args: &[String]) -> (String, Vec<String>) {
    let command = command_args[0].to_ascii_uppercase();
    let args = command_args[1..].to_vec();

    (command, args)
}
