use std::collections::HashMap;
use std::error;
use std::sync::Arc;

use tokio;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::sync::redis_value::RedisValue;
use crate::tasks::client;

pub struct Master {
    address: String,
    map: Arc<Mutex<HashMap<String, RedisValue>>>,
}

impl Master {
    pub fn init(port: u16) -> Self {
        Self {
            address: format!("127.0.0.1:{}", port),
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn error::Error>> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Listening on: {}", listener.local_addr()?);

        loop {
            let (stream, _) = listener.accept().await?;

            client::handle_client_stream(stream, Arc::clone(&self.map)).await;
        }
    }
}
