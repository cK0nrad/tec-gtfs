use dotenv::dotenv;
use std::{sync::Arc, env};

mod api;
pub mod logger;
pub mod quadtree;
pub mod store;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv().ok();

    let secret = match env::var("SECRET") {
        Ok(key) => key,
        Err(_) => panic!("No SECRET found in .env"),
    };

    let store = Arc::new(store::Store::new(&secret));
    logger::fine("FETCHER", "Loaded GTFS");
    api::init(store).await;
}
