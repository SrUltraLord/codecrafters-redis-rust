use crate::marshall::bulk_string_marshaller;

pub const NAME: &str = "ECHO";
const REQUIRED_ARGS: usize = 1;

pub fn handle(args: &Vec<String>) -> Result<String, String> {
    if args.len() != REQUIRED_ARGS {
        return Err("ERR wrong number of arguments for 'echo' command.".to_string());
    }

    Ok(bulk_string_marshaller::marshall(args[0].to_string()))
}
