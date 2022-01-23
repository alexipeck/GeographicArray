use std::{vec, collections::BTreeMap};

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

    pub fn find_nearest(&self, /* axis: Axis,  */nearest_to: &Vector) -> BTreeMap<OrderedFloat<f64>, Vector> {
        fn handle_candidates(potential_candidates: Vec<Vector>, nearest_to: &Vector, candidate: &mut BTreeMap<OrderedFloat<f64>, Vector>) {
            for g in potential_candidates.iter() {
                let cumulative_diff: f64 = g.calculate_cumulative_diff(&nearest_to);
                if cumulative_diff <= CUMULATIVE_DISTANCE_THRESHOLD {
                    candidate.insert(OrderedFloat(cumulative_diff), g.to_real());
                }
            }
        }

        let mut candidate: BTreeMap<OrderedFloat<f64>, Vector> = BTreeMap::new();
        handle_candidates(self.x[coordinate_to_index(nearest_to.x())].clone(), nearest_to, &mut candidate);
        handle_candidates(self.y[coordinate_to_index(nearest_to.y())].clone(), nearest_to, &mut candidate);
        handle_candidates(self.z[coordinate_to_index(nearest_to.z())].clone(), nearest_to, &mut candidate);
        candidate
    }

    pub fn run(&mut self) {
        println!("Generating GeographicArray and inserting values.");
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        self.insert(x, y, z);
        for _ in 0..10000000 {
            let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            self.insert(x, y, z);
        }
        println!("Done.");
        
        println!("Nearest to synthetic value: X: {}, Y: {}, Z: {}", x, y, z);
        let near_candidates = self.find_nearest(&Vector::new(x, y, z));
        for (cumulative_distance, coordinate) in near_candidates {
            println!("Distance: {:6}, X: {}, Y: {}, Z: {}", cumulative_distance.trunc(), coordinate.x(), coordinate.y(), coordinate.z());
        }

        for _ in 0..10 {
            let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);

            let h = self.find_nearest(&Vector::new(x, y, z));
            if !h.is_empty() {
                println!("Nearest to random value: X: {}, Y: {}, Z: {}", x, y, z);
                for (cumulative_distance, g) in h {
                    println!("Distance: {:6}, X: {}, Y: {}, Z: {}", cumulative_distance.trunc(), g.x(), g.y(), g.z());
                }
            } else {
                println!("No near coordinate found, needs to look at nearby indexes");
            }
        }
    }
}