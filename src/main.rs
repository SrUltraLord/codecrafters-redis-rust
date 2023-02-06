use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};

mod marshall;

use marshall::error_marshaller;
use marshall::marshaller::Marshaller;
use marshall::string_marshaller::StringMarshaller;

fn handle_connection(stream: &mut TcpStream) -> usize {
    let response = StringMarshaller::new("PONG");

    let answer = stream.write(response.marshall().as_bytes()).expect("");

    answer
}

fn main() -> Result<()> {
    println!("[Ultra Redis]");
    println!("Listening on port 6379...");

    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind to port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(&mut _stream);
            }
            Err(e) => {
                error_marshaller::marshall_error(&e);
            }
        }
    }

    Ok(())
}
