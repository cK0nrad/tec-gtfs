use std::sync::Arc;

mod api;
pub mod logger;
pub mod store;
pub mod quadtree;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let store = Arc::new(store::Store::new());
    logger::fine("FETCHER", "Loaded GTFS");
    api::init(store).await;
}
