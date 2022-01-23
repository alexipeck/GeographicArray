use std::{vec, collections::BTreeMap, time::Instant};

use rand::Rng;

use crate::{CUMULATIVE_DISTANCE_THRESHOLD, MAX_RADIUS_METERS};

use {
    std::rc::Rc,
    crate::{Vector, ZONES_USIZE, coordinate_to_index},
    ordered_float::OrderedFloat,
};

pub enum Axis {
    X,
    Y,
    Z,
}

/* pub struct XElement {

}

pub struct YElement {

}

pub struct ZElement {

} */

pub struct GeographicArray {
    pub x: Vec<Vec<Vector>>,
    //_x_median_index: usize,

    pub y: Vec<Vec<Vector>>,
    //_y_median_index: usize,

    pub z: Vec<Vec<Vector>>,
    //_z_median_index: usize,
}

impl GeographicArray {
    pub fn new(zones: usize) -> Self {
        Self {
            x: vec![Vec::new(); zones],
            y: vec![Vec::new(); zones],
            z: vec![Vec::new(); zones],
        }
    }
    pub fn default() -> Self {
        Self {
            x: vec![Vec::new(); ZONES_USIZE],
            y: vec![Vec::new(); ZONES_USIZE],
            z: vec![Vec::new(); ZONES_USIZE],
        }
    }
    
    pub fn insert(&mut self, x: f64, y: f64, z: f64) {
        let x_normalised_index: usize = coordinate_to_index(x);
        let y_normalised_index: usize = coordinate_to_index(y);
        let z_normalised_index: usize = coordinate_to_index(z);
        let x_ref = Rc::new(x);
        let y_ref = Rc::new(y);
        let z_ref = Rc::new(z);
        self.x[x_normalised_index].push(Vector::new_real_x(x, y_ref.clone(), z_ref.clone()));
        self.y[y_normalised_index].push(Vector::new_real_y(x_ref.clone(), y, z_ref.clone()));
        self.z[z_normalised_index].push(Vector::new_real_z(x_ref.clone(), y_ref.clone(), z));
    }

    pub fn find_nearest(&self, nearest_to: &Vector, deviation_limiter_radius_meters: Option<f64>) -> BTreeMap<OrderedFloat<f64>, Vector> {
        fn handle_candidates(potential_candidates: Vec<Vector>, nearest_to: &Vector, candidate: &mut BTreeMap<OrderedFloat<f64>, Vector>) {
            for g in potential_candidates.iter() {
                let cumulative_diff: f64 = g.calculate_cumulative_diff(&nearest_to);
                if cumulative_diff <= CUMULATIVE_DISTANCE_THRESHOLD {
                    candidate.insert(OrderedFloat(cumulative_diff), g.to_real());
                }
            }
        }

        let mut candidates: BTreeMap<OrderedFloat<f64>, Vector> = BTreeMap::new();
        let limiter_active: bool = deviation_limiter_radius_meters.is_some();
        let mut limiter_counter: usize = 0;
        if limiter_active {
            limiter_counter = coordinate_to_index(deviation_limiter_radius_meters.unwrap());
        }
        let mut deviation_count: usize = 0;
        while candidates.is_empty() && deviation_count < limiter_counter {
            if deviation_count == 0 {
                handle_candidates(self.x[coordinate_to_index(nearest_to.x())].clone(), nearest_to, &mut candidates);
                handle_candidates(self.y[coordinate_to_index(nearest_to.y())].clone(), nearest_to, &mut candidates);
                handle_candidates(self.z[coordinate_to_index(nearest_to.z())].clone(), nearest_to, &mut candidates);
            } else {
                handle_candidates(self.x[coordinate_to_index(nearest_to.x()) + deviation_count].clone(), nearest_to, &mut candidates);
                handle_candidates(self.y[coordinate_to_index(nearest_to.y()) + deviation_count].clone(), nearest_to, &mut candidates);
                handle_candidates(self.z[coordinate_to_index(nearest_to.z()) + deviation_count].clone(), nearest_to, &mut candidates);
                handle_candidates(self.x[coordinate_to_index(nearest_to.x()) - deviation_count].clone(), nearest_to, &mut candidates);
                handle_candidates(self.y[coordinate_to_index(nearest_to.y()) - deviation_count].clone(), nearest_to, &mut candidates);
                handle_candidates(self.z[coordinate_to_index(nearest_to.z()) - deviation_count].clone(), nearest_to, &mut candidates);
            }
            deviation_count += 1;
            if limiter_active {
                limiter_counter += 1;
            }
        }
        candidates
    }

    pub fn run(&mut self) -> u128 {
        let start_time = Instant::now();
        println!("{}: Generating GeographicArray, inserting random synthetic test value.", start_time.elapsed().as_micros());
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        self.insert(x, y, z);
        println!("{}: Inserting a few values", start_time.elapsed().as_micros());
        for _ in 0..10000000 {
            let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            self.insert(x, y, z);
        }
        println!("{}: Nearest to random synthetic value: X: {}, Y: {}, Z: {}", start_time.elapsed().as_micros(), x, y, z);
        let near_candidates = self.find_nearest(&Vector::new(x, y, z), None);
        for (cumulative_distance, coordinate) in near_candidates {
            println!("{}: Distance: {:6}, X: {}, Y: {}, Z: {}", start_time.elapsed().as_micros(), cumulative_distance.trunc(), coordinate.x(), coordinate.y(), coordinate.z());
        }

        for _ in 0..10 {
            println!("{}: Generating random values to find nearest", start_time.elapsed().as_micros());
            let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            println!("{}: Finished generating random values", start_time.elapsed().as_micros());

            let h = self.find_nearest(&Vector::new(x, y, z), Some(1000.0));
            
            if !h.is_empty() {
                println!("{}: Nearest to random value: X: {}, Y: {}, Z: {}", start_time.elapsed().as_micros(), x, y, z);
                for (cumulative_distance, g) in h {
                    println!("{}: Distance: {:6}, X: {}, Y: {}, Z: {}", start_time.elapsed().as_micros(), cumulative_distance.trunc(), g.x(), g.y(), g.z());
                    break;//Only want to run once, she'll be right
                }
            } else {
                println!("{}: Couldn't find a value within threshold", start_time.elapsed().as_micros());
            }
        }
        start_time.elapsed().as_micros()
    }
}