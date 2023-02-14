use std::error;

use tokio;
use tokio::net::TcpListener;

use crate::tasks::client;

pub struct Master {
    address: String,
}

impl Master {
    pub fn init(port: u16) -> Self {
        Self {
            address: format!("127.0.0.1:{}", port),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn error::Error>> {
        let listener = TcpListener::bind(self.address).await?;
        println!("Listening on: {}", listener.local_addr()?);

        Self::accept_connections(&listener).await?;

        Ok(())
    }

    async fn accept_connections(listener: &TcpListener) -> Result<(), Box<dyn error::Error>> {
        loop {
            let (stream, _) = listener.accept().await?;
            client::handle_client_stream(stream).await;
        }
    }
}
