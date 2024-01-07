use std::sync::Arc;

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
    gtfs: Arc<RwLock<Gtfs>>,
    stops: Arc<RwLock<QuadTree<String>>>,
    reverse_stops: Arc<RwLock<AHashMap<String, Vec<String>>>>,
    secret: String,
}

impl Store {
    pub fn new(secret: &String) -> Self {
        logger::fine("FETCHER", "Loading GTFS");
        let start_time = std::time::Instant::now();
        let gtfs = match GtfsReader::default().read("gtfs") {
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
            gtfs: Arc::new(RwLock::new(gtfs)),
            stops: Arc::new(RwLock::new(qt)),
            reverse_stops: Arc::new(RwLock::new(reverse_stops)),
            secret: secret.clone(),
        }
    }

    pub async fn refresh_gtfs(&self, secret: &String) -> Result<(), String> {
        if self.secret == "" {
            logger::fine("FETCHER", "No secret, not refreshing GTFS");
            return Err((&"Internal error").to_string());
        }

        if self.secret != *secret {
            logger::fine("FETCHER", "Wrong secret, not refreshing GTFS");
            return Err((&"Internal error").to_string());
        }

        let raw_stops = self.stops.clone();
        let raw_gtfs = self.gtfs.clone();
        let raw_reverse_stops = self.reverse_stops.clone();

        let (qt, gtfs, reverse_stops) = tokio::task::spawn_blocking(move || {
            logger::fine("FETCHER", "Loading GTFS");
            let start_time = std::time::Instant::now();
            let gtfs = match GtfsReader::default().read("gtfs") {
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
            (qt, gtfs, reverse_stops)
        })
        .await
        .unwrap();

        let mut raw_stops = raw_stops.write().await;
        *raw_stops = qt;

        let mut raw_gtfs = raw_gtfs.write().await;
        // *raw_gtfs = Gtfs::default();
        *raw_gtfs = gtfs;

        let mut raw_reverse_stops = raw_reverse_stops.write().await;
        *raw_reverse_stops = reverse_stops;
        Ok(())
    }

    pub fn get_gtfs(&self) -> Arc<RwLock<Gtfs>> {
        self.gtfs.clone()
    }

    pub fn get_stops(&self) -> Arc<RwLock<QuadTree<String>>> {
        self.stops.clone()
    }

    pub fn get_reverse_stops(&self) -> Arc<RwLock<AHashMap<String, Vec<String>>>> {
        self.reverse_stops.clone()
    }
}
