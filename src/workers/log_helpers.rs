use std::f64::consts::PI;
use std::hash::Hasher;
use std::hash::Hash;

pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371e3; // Earth's radius in meters
    let phi1 = lat1 * PI / 180.0;
    let phi2 = lat2 * PI / 180.0;
    let delta_phi = (lat2 - lat1) * PI / 180.0;
    let delta_lambda = (lon2 - lon1) * PI / 180.0;

    let a = (delta_phi / 2.0).sin().powi(2)
          + phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c // Distance in meters
}

pub fn calculate_advisory_lock_key(canonical_name: &str) -> u32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    canonical_name.hash(&mut hasher);
    // Get the lower 32 bits of the hash
    hasher.finish() as u32
}