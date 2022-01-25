use rand::seq::index::IndexVec;

use crate::{Vector, CUMULATIVE_DISTANCE_THRESHOLD, IndexVector, coordinate_to_index_x, coordinate_to_index_y, coordinate_to_index_z};

use {
    crate::{ReferenceVector, ZONES_USIZE},
    ordered_float::OrderedFloat,
    std::{collections::BTreeMap, rc::Rc, time::Instant, vec},
};

pub enum Axis {
    X,
    Y,
    Z,
}

pub struct GeographicArray {
    pub x: Vec<Vec<ReferenceVector>>,
    x_median_index: usize,
    pub y: Vec<Vec<ReferenceVector>>,
    y_median_index: usize,
    pub z: Vec<Vec<ReferenceVector>>,
    z_median_index: usize,
}

impl GeographicArray {
    pub fn new(zones: usize) -> Self {
        Self {
            x: vec![Vec::new(); zones],
            x_median_index: zones / 2,
            y: vec![Vec::new(); zones],
            y_median_index: zones / 2,
            z: vec![Vec::new(); zones],
            z_median_index: zones / 2,
        }
    }
    pub fn default() -> Self {
        Self {
            x: vec![Vec::new(); ZONES_USIZE],
            x_median_index: ZONES_USIZE / 2,
            y: vec![Vec::new(); ZONES_USIZE],
            y_median_index: ZONES_USIZE / 2,
            z: vec![Vec::new(); ZONES_USIZE],
            z_median_index: ZONES_USIZE / 2,
        }
    }

    pub fn insert(&mut self, vector: Vector) {
        let x_normalised_index: usize = coordinate_to_index_x(vector.x);
        let y_normalised_index: usize = coordinate_to_index_y(vector.y);
        let z_normalised_index: usize = coordinate_to_index_z(vector.z);
        let x_ref = Rc::new(vector.x);
        let y_ref = Rc::new(vector.y);
        let z_ref = Rc::new(vector.z);
        self.x[x_normalised_index].push(ReferenceVector::new_real_x(
            vector.x,
            y_ref.clone(),
            z_ref.clone(),
        ));
        self.y[y_normalised_index].push(ReferenceVector::new_real_y(
            x_ref.clone(),
            vector.y,
            z_ref.clone(),
        ));
        self.z[z_normalised_index].push(ReferenceVector::new_real_z(
            x_ref.clone(),
            y_ref.clone(),
            vector.z,
        ));
    }

    pub fn find_nearest(
        &self,
        nearest_to: &Vector,
        deviation_limiter_radius_meters: Option<&Vector>,
    ) -> BTreeMap<OrderedFloat<f64>, ReferenceVector> {
        fn handle_candidates(
            potential_candidates: Vec<ReferenceVector>,
            nearest_to: &Vector,
            candidate: &mut BTreeMap<OrderedFloat<f64>, ReferenceVector>,
        ) {
            for g in potential_candidates.iter() {
                let cumulative_diff: f64 = g.calculate_cumulative_diff(&nearest_to);
                if cumulative_diff <= CUMULATIVE_DISTANCE_THRESHOLD {
                    candidate.insert(OrderedFloat(cumulative_diff), g.to_real());
                }
            }
        }


        //limiter counter currently can't deal with differentiating x, y, z distances, z is the most likely to fuck it up
        //I could make x, y, z searches run independantly, but it would need to store previous iterations data, such as the whole
        //pool of collected values to cross reference once any given axis has met it's search threshold, this should save a significant amount
        //of time over searching each directtion at the same rate.


        let mut candidates: BTreeMap<OrderedFloat<f64>, ReferenceVector> = BTreeMap::new();
        let limiter_active: bool = deviation_limiter_radius_meters.is_some();
        let mut limiter_threshold: IndexVector = IndexVector::new(0, 0, 0);
        
        if limiter_active {
            limiter_threshold = IndexVector::from_vector(deviation_limiter_radius_meters.unwrap());
        }
        let mut deviation_count: usize = 0;
        let index_vector: IndexVector = IndexVector::from_vector(nearest_to);
        let limiter_counter_largest_bound = limiter_threshold.max_index();
        while candidates.is_empty() && deviation_count > limiter_counter_largest_bound {
            if deviation_count == 0 {
                if deviation_count <= limiter_threshold.x {
                    handle_candidates(
                        self.x[index_vector.x].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                }
                if deviation_count <= limiter_threshold.y {
                    handle_candidates(
                        self.y[index_vector.y].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                }

                if deviation_count <= limiter_threshold.z {
                    handle_candidates(
                        self.z[index_vector.z].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                }
            } else {
                //x
                if deviation_count <= limiter_threshold.x {
                    handle_candidates(self.x[index_vector.x + deviation_count].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                    handle_candidates(
                        self.x[index_vector.x - deviation_count].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                }

                //y
                if deviation_count <= limiter_threshold.y {
                    handle_candidates(
                        self.y[index_vector.y + deviation_count].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                    handle_candidates(
                        self.y[index_vector.y - deviation_count].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                }

                //z
                if deviation_count <= limiter_threshold.z {
                    handle_candidates(
                        self.z[index_vector.z + deviation_count].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                    handle_candidates(
                        self.z[index_vector.z - deviation_count].clone(),
                        nearest_to,
                        &mut candidates,
                    );
                }
            }
            deviation_count += 1;
        }
        candidates
    }

    pub fn run(&mut self) -> u128 {
        let start_time = Instant::now();
        println!(
            "{}: Generating GeographicArray.",
            start_time.elapsed().as_micros()
        );
        let mut rng = rand::thread_rng();
        let synthetic_value: Vector = Vector::generate_random_seeded(&mut rng);
        self.insert(synthetic_value.clone());
        let values_to_insert: usize = 10000000;
        println!(
            "{}: Inserting a few values: {}",
            start_time.elapsed().as_micros(),
            values_to_insert,
        );
        for _ in 0..values_to_insert {
            self.insert(Vector::generate_random_seeded(&mut rng));
        }
        println!(
            "{}: Nearest to random synthetic value: X: {}, Y: {}, Z: {}",
            start_time.elapsed().as_micros(),
            synthetic_value.x,
            synthetic_value.y,
            synthetic_value.z,
        );
        let near_candidates = self.find_nearest(&synthetic_value, None);
        for (cumulative_distance, coordinate) in near_candidates {
            println!(
                "{}: Distance: {:17}, X: {}, Y: {}, Z: {}",
                start_time.elapsed().as_micros(),
                cumulative_distance,
                coordinate.x(),
                coordinate.y(),
                coordinate.z()
            );
        }

        for _ in 0..10 {
            println!(
                "{}: Generating random values to find nearest",
                start_time.elapsed().as_micros()
            );
            let random_vector: Vector = Vector::generate_random_seeded(&mut rng);
            println!(
                "{}: Finished generating random values",
                start_time.elapsed().as_micros()
            );

            let ordered_candidates = self.find_nearest(&random_vector, Some(&Vector::new(1000.0, 1000.0, 1000.0)));

            if !ordered_candidates.is_empty() {
                println!(
                    "{}: Nearest to random value: X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    random_vector.x,
                    random_vector.y,
                    random_vector.z
                );
                for (cumulative_distance, g) in ordered_candidates {
                    println!(
                        "{}: Distance: {:17}, X: {}, Y: {}, Z: {}",
                        start_time.elapsed().as_micros(),
                        cumulative_distance,
                        g.x(),
                        g.y(),
                        g.z()
                    );
                    break; //Only want to run once, she'll be right
                }
            } else {
                println!(
                    "{}: Couldn't find a value within threshold",
                    start_time.elapsed().as_micros()
                );
            }
        }

        start_time.elapsed().as_micros()
    }
}
