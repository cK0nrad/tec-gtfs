use super::TripQuery;
use crate::store::Store;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use gtfs_structures::Shape;
use serde_json::json;
use std::sync::Arc;

pub async fn shape(State(app): State<Arc<Store>>, query: Query<TripQuery>) -> impl IntoResponse {
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

    let shape_id = match &trip.shape_id {
        Some(shape_id) => shape_id.clone(),
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "no shape"})))),
    };

    let shape: Vec<Shape> = match app.get_shape(&shape_id) {
        Ok(shape) => {
            let mut vec = Vec::with_capacity(shape.len());
            for s in shape {
                let mut def = Shape::default();
                def.id = s.id.clone();
                def.latitude = s.latitude.clone();
                def.longitude = s.longitude.clone();
                def.sequence = s.sequence.clone();
                def.dist_traveled = s.dist_traveled.clone();
                vec.push(def);
            }
            vec
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid shape_id"})),
            ))
        }
    };

    Ok(Json(shape).into_response())
}
