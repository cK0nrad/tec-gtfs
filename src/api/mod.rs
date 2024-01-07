use crate::{logger, store::Store};
use axum::{http::Method, routing::get, Router};
use std::{sync::Arc, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

mod info;
mod shape;
mod stops;
mod theorical;
mod gtfs;

pub async fn init(store: Arc<Store>) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PUT])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/theorical", get(theorical::theorical_schedule))
        .route("/shape", get(shape::shape))
        .route("/info", get(info::info))
        .route("/stops", get(stops::stops))
        .route("/bus_from_stop", get(stops::bus_per_stop))
        .route("/refresh_gtfs", get(gtfs::refresh))
        .layer(cors)
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3006").await.unwrap();
    logger::fine(
        "WEBSERVER",
        format!("Listening to: {}", "0.0.0.0:3006").as_str(),
    );

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

#[derive(serde::Deserialize)]
pub struct BboxQuery {
    north: Option<f32>,
    east: Option<f32>,
    west: Option<f32>,
    south: Option<f32>,
}

#[derive(serde::Deserialize)]
pub struct TripQuery {
    trip_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct StopQuery {
    stop_id: Option<String>,
}
