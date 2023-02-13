use std::error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::marshall::marshaller::Marshaller;
use crate::marshall::string_marshaller::StringMarshaller;

// This struct handles client's connection
pub struct ClientListener<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> ClientListener<'a> {
    pub fn init(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    pub async fn handle_client_stream(
        &mut self,
        buffer: &mut [u8; 1024],
    ) -> Result<(), Box<dyn error::Error>> {
        loop {
            let bytes_read = self.stream.read(buffer).await?;

            if bytes_read == 0 {
                break;
            }

            let serialized_response = StringMarshaller::init().marshall("PONG");

            self.stream.write(serialized_response.as_bytes()).await?;
        }

        Ok(())
    }
}
