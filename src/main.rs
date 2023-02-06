use std::io::Result;
use std::net::TcpListener;

fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379")
        .expect("Could not bind to port 6379");

    for stream in listener.incoming() {
        match stream  {
            Ok(_stream) => {
                println!("Accepting connection!")
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
