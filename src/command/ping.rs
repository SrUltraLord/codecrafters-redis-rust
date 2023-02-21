use crate::marshall::string_marshaller;

pub const NAME: &str = "PING";
const DEFAULT_RESPONSE: &str = "PONG";

pub fn handle(args: &Vec<String>) -> String {
    let mut response = DEFAULT_RESPONSE.to_string();

    if args.len() != 0 {
        response = args[0].to_string();
    }

    string_marshaller::marshall(response)
}
