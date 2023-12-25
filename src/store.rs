use ahash::AHashMap;
use gtfs_structures::{Gtfs, GtfsReader};
use serde::Serialize;
use tokio::sync::RwLock;

use crate::{
    logger,
    quadtree::{Coordinate, Extent, QuadTree},
};

#[derive(Serialize)]
pub struct Bus {
    id: String,
    line: String,
    line_id: String,
    latitude: f32,
    longitude: f32,
    speed: f32,
    last_update: u64,
}

impl Bus {
    pub fn new(
        id: String,
        line: String,
        line_id: String,
        latitude: f32,
        longitude: f32,
        speed: f32,
        last_update: u64,
    ) -> Self {
        Self {
            id,
            line,
            line_id,
            latitude,
            longitude,
            speed,
            last_update,
        }
    }
}

pub struct Store {
    pub gtfs: RwLock<Gtfs>,
    pub stops: RwLock<QuadTree<String>>,
    pub reverse_stops: RwLock<AHashMap<String, Vec<String>>>,
}

impl Store {
    pub fn new() -> Self {
        logger::fine("FETCHER", "Loading GTFS");
        let start_time = std::time::Instant::now();
        let gtfs = match GtfsReader::default().read("src/gtfs") {
            Ok(gtfs) => gtfs,
            Err(_) => panic!("Error loading gtfs"),
        };
        logger::fine(
            "FETCHER",
            &format!("Loaded GTFS: [{:?}]", start_time.elapsed()),
        );

        logger::fine("FETCHER", "Loading stops cache");
        let ext = Extent::new(2.285, 49.063, 7.053, 51.775);
        let mut qt: QuadTree<String> = QuadTree::<String>::new(ext);

        let start_time = std::time::Instant::now();
        for (stop_id, val) in gtfs.stops.iter() {
            match (val.latitude, val.longitude) {
                (Some(lat), Some(lon)) => {
                    qt.insert(&Coordinate::new(lon, lat), stop_id.clone());
                }
                _ => continue,
            }
        }
        logger::fine(
            "FETCHER",
            &format!("Loaded stops cache: [{:?}]", start_time.elapsed()),
        );

        logger::fine("FETCHER", "Loading reverse stops cache");
        let mut reverse_stops: AHashMap<String, Vec<String>> = AHashMap::new();
        let start_time = std::time::Instant::now();
        // for (_, val) in gtfs.trips.iter() {
        //     let route_id = val.route_id.clone();
        //     for st in &val.stop_times {
        //         let stop_name = st.stop.id.clone();
        //         if !reverse_stops.contains_key(&stop_name) {
        //             reverse_stops.insert(stop_name.clone(), Vec::new());
        //         }
        //         reverse_stops.get_mut(&stop_name).unwrap().push(route_id.clone());
        //     }
        // }

        for (_, val) in gtfs.trips.iter() {
            let route_id = val.route_id.clone();
            for st in &val.stop_times {
                let stop_name = st.stop.id.clone();
                if !reverse_stops.contains_key(&stop_name) {
                    reverse_stops.insert(stop_name.clone(), Vec::new());
                }
                let vec = reverse_stops.get_mut(&stop_name).unwrap(); //safe because we just inserted it

                if !vec.contains(&route_id) {
                    vec.push(route_id.clone());
                }
            }
        }

        logger::fine(
            "FETCHER",
            &format!("Loaded reverse stops cache: [{:?}]", start_time.elapsed()),
        );

        Self {
            gtfs: RwLock::new(gtfs),
            stops: RwLock::new(qt),
            reverse_stops: RwLock::new(reverse_stops),
        }
    }
}
