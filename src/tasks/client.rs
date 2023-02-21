use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::command::command_handler;

pub async fn handle_client_stream(mut stream: TcpStream) {
    tokio::spawn(async move {
        let mut buffer: [u8; 1024] = [0; 1024];

        loop {
            let bytes_read = stream.read(&mut buffer).await.unwrap();

            if bytes_read == 0 {
                break;
            }

            let serialized_response =
                command_handler::handle_command_from_buffer(&buffer[..bytes_read].to_owned());

            stream.write(serialized_response.as_bytes()).await.unwrap();
        }
    });
}
