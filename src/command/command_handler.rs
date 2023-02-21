use crate::command::{echo, ping};

use crate::marshall::{array_marshaller, error_marshaller};

pub fn handle_command_from_buffer(buffer: &[u8]) -> String {
    let command_args = array_marshaller::unmarshall_from_buffer(buffer);
    let (command, args) = parse_array_as_command(command_args.as_slice());

    match command.as_str() {
        ping::NAME => ping::handle(&args),
        echo::NAME => echo::handle(&args),
        _ => {
            error_marshaller::marshall(format!("ERR unkown command '{}'.", command.to_lowercase()))
        }
    }
}

fn parse_array_as_command(command_args: &[String]) -> (String, Vec<String>) {
    let command = command_args[0].to_ascii_uppercase();
    let args = command_args[1..].to_vec();

    (command, args)
}
