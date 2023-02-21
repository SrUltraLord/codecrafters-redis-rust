use crate::marshall::{bulk_string_marshaller, error_marshaller};

pub const NAME: &str = "ECHO";

pub fn handle(args: &Vec<String>) -> String {
    if args.len() != 1 {
        return error_marshaller::marshall(
            "ERR wrong number of arguments for 'echo' command.".to_string(),
        );
    }

    bulk_string_marshaller::marshall(args[0].to_string())
}
