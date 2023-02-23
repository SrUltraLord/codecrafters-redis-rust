use crate::marshall::string_marshaller;

pub const NAME: &str = "PING";
const DEFAULT_RESPONSE: &str = "PONG";
const MAX_ARGS: usize = 1;
const REQUIRED_ARGS: usize = 0;

pub fn handle(args: &Vec<String>) -> Result<String, String> {
    let mut response = DEFAULT_RESPONSE.to_string();
    let total_args = args.len();

    if total_args != MAX_ARGS && total_args != REQUIRED_ARGS {
        return Err("Err wrong number of arguments for 'ping' command".to_string());
    }

    if total_args == MAX_ARGS {
        response = args[0].to_string();
    }

    Ok(string_marshaller::marshall(response))
}
