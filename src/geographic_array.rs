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
    _x_median_index: usize,
    pub y: Vec<Vec<ReferenceVector>>,
    _y_median_index: usize,
    pub z: Vec<Vec<ReferenceVector>>,
    _z_median_index: usize,
}

impl GeographicArray {
    pub fn new(zones: usize) -> Self {
        Self {
            x: vec![Vec::new(); zones],
            _x_median_index: zones / 2,
            y: vec![Vec::new(); zones],
            _y_median_index: zones / 2,
            z: vec![Vec::new(); zones],
            _z_median_index: zones / 2,
        }
    }
    pub fn default() -> Self {
        Self {
            x: vec![Vec::new(); ZONES_USIZE],
            _x_median_index: ZONES_USIZE / 2,
            y: vec![Vec::new(); ZONES_USIZE],
            _y_median_index: ZONES_USIZE / 2,
            z: vec![Vec::new(); ZONES_USIZE],
            _z_median_index: ZONES_USIZE / 2,
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
            z_ref,
        ));
        self.z[z_normalised_index].push(ReferenceVector::new_real_z(
            x_ref,
            y_ref,
            vector.z,
        ));
    }

    fn managed_search(
        &self,
        candidates: &mut BTreeMap<OrderedFloat<f64>, ReferenceVector>,
        deviation_count: &usize,
        limiter_threshold: &usize,
        axis: &Axis,
        search_index: usize,
        nearest_to: &Vector,
    ) {
        if deviation_count <= limiter_threshold {
            let potential_candidates: Vec<ReferenceVector> = match axis {
                Axis::X => self.x[search_index].clone(),
                Axis::Y => self.y[search_index].clone(),
                Axis::Z => self.z[search_index].clone(),
            };
            for reference_vector in potential_candidates.iter() {
                let cumulative_diff: f64 = reference_vector.calculate_cumulative_diff(nearest_to);
                if cumulative_diff <= CUMULATIVE_DISTANCE_THRESHOLD {
                    candidates.insert(OrderedFloat(cumulative_diff), reference_vector.to_real());
                }
            }
        } else {
            let length = match axis {
                Axis::X => self.x[search_index].len(),
                Axis::Y => self.y[search_index].len(),
                Axis::Z => self.z[search_index].len(),
            };
            println!("{} ignored values.", length);
        }
    }

    pub fn find_nearest(
        &self,
        nearest_to: &Vector,
        deviation_limiter_radius_meters: Option<&Vector>,
    ) -> BTreeMap<OrderedFloat<f64>, ReferenceVector> {


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
        
        let axis_x = &Axis::X;
        let axis_y = &Axis::Y;
        let axis_z = &Axis::Z;

        //still doesn't deal with what happens when it gets to either end of the vector, it should just stop the search on that axis
        //I need to check how good it is at knowing how far it can actually move when it's near the edge, it might actually be inherantly handled
        while candidates.is_empty() && deviation_count < limiter_counter_largest_bound {
            //neutral & positive
            self.managed_search(&mut candidates, &deviation_count, &limiter_threshold.x, axis_x, index_vector.x + deviation_count, nearest_to);
            self.managed_search(&mut candidates, &deviation_count, &limiter_threshold.y, axis_y, index_vector.y + deviation_count, nearest_to);
            self.managed_search(&mut candidates, &deviation_count, &limiter_threshold.z, axis_z, index_vector.z + deviation_count, nearest_to);
            
            //negative
            if deviation_count > 0 {
                self.managed_search(&mut candidates, &deviation_count, &limiter_threshold.x, axis_x, index_vector.x - deviation_count, nearest_to);
                self.managed_search(&mut candidates, &deviation_count, &limiter_threshold.y, axis_y, index_vector.y - deviation_count, nearest_to);
                self.managed_search(&mut candidates, &deviation_count, &limiter_threshold.z, axis_z, index_vector.z - deviation_count, nearest_to);
            }
            deviation_count += 1;
        }
        println!("Found {} candidates.", candidates.len());
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

        //move to testing.rs
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

            for (cumulative_distance, reference_vector) in ordered_candidates.iter() {
                println!(
                    "{}: Nearest to random value: X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    random_vector.x,
                    random_vector.y,
                    random_vector.z
                );
                println!(
                    "{}: Distance: {:17}, X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    cumulative_distance,
                    reference_vector.x(),
                    reference_vector.y(),
                    reference_vector.z()
                );
            }

            if ordered_candidates.is_empty() {
                println!(
                    "{}: Couldn't find a value within threshold",
                    start_time.elapsed().as_micros()
                );
            }
        }

        start_time.elapsed().as_micros()
    }
}
