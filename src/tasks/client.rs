use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::marshall::marshaller::Marshaller;
use crate::marshall::string_marshaller::StringMarshaller;

pub async fn handle_client_stream(mut stream: TcpStream) {
    tokio::spawn(async move {
        let mut buffer: [u8; 1024] = [0; 1024];

        loop {
            let bytes_read = stream.read(&mut buffer).await.unwrap();

            if bytes_read == 0 {
                break;
            }

            let serialized_response = StringMarshaller::init().marshall("PONG");

            stream.write(serialized_response.as_bytes()).await.unwrap();
        }
    });
}
