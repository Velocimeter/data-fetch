use tracing::info;

use dotenv::dotenv;

mod server;
use server::server;
mod syncer;
use syncer::syncer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    info!("Tracing initialized");

    tokio::spawn(syncer());

    server().await;
}
