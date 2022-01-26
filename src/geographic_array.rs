use crate::{Vector, CUMULATIVE_DISTANCE_THRESHOLD, IndexVector, coordinate_to_index_x, coordinate_to_index_y, coordinate_to_index_z, Axis, DynamicSearchValidated, Candidates};

use {
    crate::{ReferenceVector, ZONES_USIZE},
    ordered_float::OrderedFloat,
    std::{collections::BTreeMap, rc::Rc, time::Instant, vec},
};

pub struct GeographicArray {
    pub x: Vec<Vec<ReferenceVector>>,
    //_x_median_index: usize,
    pub y: Vec<Vec<ReferenceVector>>,
    //_y_median_index: usize,
    pub z: Vec<Vec<ReferenceVector>>,
    //_z_median_index: usize,
}

impl GeographicArray {
    pub fn new(zones: usize) -> Self {
        Self {
            x: vec![Vec::new(); zones],
            //_x_median_index: zones / 2,
            y: vec![Vec::new(); zones],
            //_y_median_index: zones / 2,
            z: vec![Vec::new(); zones],
            //_z_median_index: zones / 2,
        }
    }
    pub fn default() -> Self {
        Self {
            x: vec![Vec::new(); ZONES_USIZE],
            //_x_median_index: ZONES_USIZE / 2,
            y: vec![Vec::new(); ZONES_USIZE],
            //_y_median_index: ZONES_USIZE / 2,
            z: vec![Vec::new(); ZONES_USIZE],
            //_z_median_index: ZONES_USIZE / 2,
        }
    }

    pub fn insert(&mut self, vector: Vector) -> IndexVector {
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
        IndexVector::new(x_normalised_index, y_normalised_index, z_normalised_index)
    }

    /* //returns whether work was done
    //if it wasn't allowed to search because of limiter, no work was done.
    //if it was allowed to search, but found nothing, work was done.
    pub fn index_search(&self, search_package: SearchPackage, candidates: &mut Candidates) {
        if search_package.index_within_limits() {
            let potential_candidates;
            match search_package.index {
                Axis::XIndex(index) => {
                    potential_candidates = self.x[index].clone();
                },
                Axis::YIndex(index) => {
                    potential_candidates = self.y[index].clone();
                },
                Axis::ZIndex(index) => {
                    potential_candidates = self.z[index].clone();
                },
                _ => panic!(),
            }
            if let Axis::Vector(nearest_to) = search_package.coordinate {
                for reference_vector in potential_candidates.iter() {
                    let cumulative_diff: f64 = reference_vector.calculate_cumulative_diff(&nearest_to);
                    if cumulative_diff <= CUMULATIVE_DISTANCE_THRESHOLD {
                        candidates.insert(OrderedFloat(cumulative_diff), reference_vector.to_real());
                    }
                }
            } else {
                panic!();
            }
        }
    } */

    //returns whether work was done
    //if it wasn't allowed to search because of limiter, no work was done.
    //if it was allowed to search, but found nothing, work was done.
    /* pub fn deviated_search(value: SearchPackage) -> bool {
        true
    } */

    /* fn managed_search(
        &self,
        candidates: &mut BTreeMap<OrderedFloat<f64>, ReferenceVector>,
        deviation_count: &usize,
        search_package: &SearchPackage,
        search_index: usize,
        nearest_to: &Vector,
    ) {
        self.index_search(, );



        
        if zone_axis_range.index_within_limits(search_index) {
            let potential_candidates: Vec<ReferenceVector> = match axis {
                Axis::X => self.x[search_index].clone(),
                Axis::Y => self.y[search_index].clone(),
                Axis::Z => self.z[search_index].clone(),
            };
        }
        if *limiter_threshold == 0 || deviation_count <= limiter_threshold {
            
            for reference_vector in potential_candidates.iter() {
                let cumulative_diff: f64 = reference_vector.calculate_cumulative_diff(nearest_to);
                if cumulative_diff <= CUMULATIVE_DISTANCE_THRESHOLD {
                    candidates.insert(OrderedFloat(cumulative_diff), reference_vector.to_real());
                }
            }
        } else {
            let length = match axis {
                AxisSpecific::X => self.x[search_index].len(),
                AxisSpecific::Y => self.y[search_index].len(),
                AxisSpecific::Z => self.z[search_index].len(),
            };
            println!("{} ignored values.", length);
        }
    } */

    /* pub fn get_values_from_specific_index(&self, axis: Axis, index: usize) -> BTreeMap<OrderedFloat<f64>, ReferenceVector> {
        let mut values: BTreeMap<OrderedFloat<f64>, ReferenceVector> = BTreeMap::new();
        self.managed_search(&mut values, &0, &limiter_threshold.x, axis_x, index_vector.x + deviation_count, nearest_to);
        self.managed_search(&mut values, &0, &limiter_threshold.y, axis_y, index_vector.y + deviation_count, nearest_to);
        self.managed_search(&mut values, &0, &limiter_threshold.z, axis_z, index_vector.z + deviation_count, nearest_to);
    } */

    //TODO: Make the range in KM relative to real distances rather than indexes
    pub fn find_nearest(
        &self,
        nearest_to: &Vector,
    ) -> Candidates {
        let x_axis: &Axis = &Axis::X;
        let y_axis: &Axis = &Axis::Y;
        let z_axis: &Axis = &Axis::Z;

        let nearest_to_index_vector = IndexVector::from_vector(nearest_to);

        let x_dynamic_search_order = DynamicSearchValidated::new(x_axis, nearest_to, nearest_to_index_vector.x, None);
        let y_dynamic_search_order = DynamicSearchValidated::new(y_axis, nearest_to, nearest_to_index_vector.y, None);
        let z_dynamic_search_order = DynamicSearchValidated::new(z_axis, nearest_to, nearest_to_index_vector.z, None);
        let mut candidates: Candidates = BTreeMap::new();
        x_dynamic_search_order.run(self, &mut candidates);
        y_dynamic_search_order.run(self, &mut candidates);
        z_dynamic_search_order.run(self, &mut candidates);

        if candidates.is_empty() {

        }
        
        return candidates;

       /*  /* if limiter_active {
            limiter_threshold = IndexVector::from_vector(deviation_limiter_radius_meters.unwrap());
        } */
        let mut deviation_count: usize = 0;
        let index_vector: IndexVector = IndexVector::from_vector(nearest_to);
        let x_positive_limit = ZONES_INDEXED_USIZE - index_vector.x;//negative limit is itself
        let y_positive_limit = ZONES_INDEXED_USIZE - index_vector.y;//negative limit is itself
        let z_positive_limit = ZONES_INDEXED_USIZE - index_vector.z;//negative limit is itself
        println!("x_positive_limit: {}:{}", x_positive_limit, ZONES_INDEXED_USIZE);
        println!("y_positive_limit: {}", y_positive_limit);
        println!("z_positive_limit: {}", z_positive_limit);
        /* } else {
            let index_vector = IndexVector::from_vector(deviation_limiter_radius_meters.unwrap());
            x_positive_limit = ZONES_INDEXED_USIZE - index_vector.x;
            y_positive_limit = ZONES_INDEXED_USIZE - index_vector.y;
            z_positive_limit = ZONES_INDEXED_USIZE - index_vector.z;
            println!("x_positive_limit: {}:{}", x_positive_limit, ZONES_INDEXED_USIZE);
            println!("y_positive_limit: {}", y_positive_limit);
            println!("z_positive_limit: {}", z_positive_limit);
            
        } */
        assert!(x_positive_limit <= ZONES_INDEXED_USIZE);
        assert!(y_positive_limit <= ZONES_INDEXED_USIZE);
        assert!(z_positive_limit <= ZONES_INDEXED_USIZE);

        if let Some(expected_index) = expected_index {
            if *expected_index != index_vector {
                println!("Expected: X: {}, Y: {}, Z: {},\n
                          Actual  : X: {}, Y: {}, Z: {}",
                        expected_index.x, expected_index.y, expected_index.z,
                        index_vector.x, index_vector.y, index_vector.z,
                    );
            }
        }

        //let limiter_counter_largest_bound = limiter_threshold.max_index();
        
        /* let axis_x = &Axis::X;
        let axis_y = &Axis::Y;
        let axis_z = &Axis::Z; */

        //still doesn't deal with what happens when it gets to either end of the vector, it should just stop the search on that axis
        //I need to check how good it is at knowing how far it can actually move when it's near the edge, it might actually be inherantly handled
        let pre_calc_condition: bool = true;
        while candidates.is_empty()/*  && deviation_count < limiter_counter_largest_bound */ {
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
        println!("Found {} candidate(s).", candidates.len());
        candidates */
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
        let near_candidates = self.find_nearest(&synthetic_value);
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

            let ordered_candidates = self.find_nearest(&random_vector);

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
