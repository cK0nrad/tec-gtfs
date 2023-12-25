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

pub async fn theorical_schedule(
    State(app): State<Arc<Store>>,
    query: Query<TripQuery>,
) -> impl IntoResponse {
    let app = app.gtfs.read().await;

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
        Ok(trip) => trip.clone(),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid trip_id"})),
            ))
        }
    };

    Ok(Json(trip).into_response())
}
