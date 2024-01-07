use super::TripQuery;
use crate::store::Store;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn info(State(app): State<Arc<Store>>, query: Query<TripQuery>) -> impl IntoResponse {
    let gtfs = app.get_gtfs();
    let app = gtfs.read().await;
    let trip_id = match &query.trip_id {
        Some(trip_id) => trip_id,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing trip_id"})),
            ))
        }
    };

    let trip = match app.get_trip(trip_id) {
        Ok(trip) => trip,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid trip_id"})),
            ))
        }
    };

    let route = match app.get_route(&trip.route_id) {
        Ok(route) => route,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid trip_id"})),
            ))
        }
    };

    let json = json!({
        "route_long_name": route.long_name,
        "route_direction": trip.direction_id
    });

    Ok(Json(json).into_response())
}
