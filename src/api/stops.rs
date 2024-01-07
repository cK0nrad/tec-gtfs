use super::{BboxQuery, StopQuery};
use crate::{quadtree::Extent, store::Store};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn stops(State(app): State<Arc<Store>>, query: Query<BboxQuery>) -> impl IntoResponse {
    let north = match &query.north {
        Some(north) => north,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing north"})),
            ))
        }
    };

    let south = match &query.south {
        Some(south) => south,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing south"})),
            ))
        }
    };

    let east = match &query.east {
        Some(east) => east,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing east"})),
            ))
        }
    };

    let west = match &query.west {
        Some(west) => west,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing west"})),
            ))
        }
    };

    let stops = app.get_stops();
    let app = stops.read().await;
    let extent = Extent::new(*west as f64, *south as f64, *east as f64, *north as f64);
    let stops = app.find_bbox(&extent);

    Ok(Json(stops).into_response())
}

pub async fn bus_per_stop(
    State(app): State<Arc<Store>>,
    query: Query<StopQuery>,
) -> impl IntoResponse {
    let stop_id = match &query.stop_id {
        Some(stop_id) => stop_id,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing stop_id"})),
            ))
        }
    };

    let rs = app.get_reverse_stops();
    let app = rs.read().await;
    match app.get(stop_id) {
        Some(stops) => Ok(Json(stops).into_response()),
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Stop not found"})),
        )),
    }
}
