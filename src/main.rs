use std::env;
use std::error::Error;

mod marshall;
mod tasks;

use crate::tasks::master;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port: u16 = env::args()
        .nth(1)
        .unwrap_or("6379".to_string())
        .parse::<u16>()
        .expect("Invalid port arg");

    master::Master::init(port).run().await?;

    Ok(())
}
