use std::cmp::max;

use ordered_float::OrderedFloat;

use crate::{Vector, IndexVector, Axis, DynamicSearchValidated, Candidates, SearchMode, coordinate_to_index_x, coordinate_to_index_y, coordinate_to_index_z, coordinate_to_index, IndexRange, max_f64};

use {
    crate::{ReferenceVector, ZONES_USIZE},
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
        let x_normalised_index: usize = coordinate_to_index_x(&vector.x);
        let y_normalised_index: usize = coordinate_to_index_y(&vector.y);
        let z_normalised_index: usize = coordinate_to_index_z(&vector.z);
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

    //TODO: Make the range in KM relative to real distances rather than indexes
    //this function returns more than one value because the extra data it returns took no extra work to attain
    //There will be a function that only returns one value available
    //as well as an experimental version where you only need to search one axis
    pub fn find_nearest(
        &self,
        nearest_to: &Vector,
    ) -> Candidates {
        let x_axis: &Axis = &Axis::X;
        let y_axis: &Axis = &Axis::Y;
        let z_axis: &Axis = &Axis::Z;

        let nearest_to_index_vector = IndexVector::from_vector(nearest_to);
        let search_mode_nearest = &SearchMode::Nearest;
        let x_dynamic_search_order = DynamicSearchValidated::new(x_axis, nearest_to, nearest_to_index_vector.x, search_mode_nearest);
        let y_dynamic_search_order = DynamicSearchValidated::new(y_axis, nearest_to, nearest_to_index_vector.y, search_mode_nearest);
        let z_dynamic_search_order = DynamicSearchValidated::new(z_axis, nearest_to, nearest_to_index_vector.z, search_mode_nearest);
        let mut candidates: Candidates = BTreeMap::new();
        x_dynamic_search_order.run(self, &mut candidates);
        y_dynamic_search_order.run(self, &mut candidates);
        z_dynamic_search_order.run(self, &mut candidates);
        
        candidates
    }

    //the axis chosen shouldn't actually matter, at this point, I believe the chosen axis is arbitrary if a potential full search of the axis is acceptable
    pub fn experimental_find_nearest(
        &self,
        nearest_to: &Vector,
        axis: &Axis,
    ) -> Candidates {
        let nearest_to_index_vector = IndexVector::from_vector(nearest_to);
        let search_mode_nearest = &SearchMode::Nearest;
        let mut candidates: Candidates = BTreeMap::new();
        match axis {
            Axis::X => {
                let x_dynamic_search_order = DynamicSearchValidated::new(&Axis::X, nearest_to, nearest_to_index_vector.x, search_mode_nearest);
                x_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Y => {
                let y_dynamic_search_order = DynamicSearchValidated::new(&Axis::Y, nearest_to, nearest_to_index_vector.y, search_mode_nearest);
                y_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Z => {
                let z_dynamic_search_order = DynamicSearchValidated::new(&Axis::Z, nearest_to, nearest_to_index_vector.z, search_mode_nearest);
                z_dynamic_search_order.run(self, &mut candidates);
            },
        }
        candidates
    }

    //the axis chosen shouldn't actually matter, at this point, I believe the chosen axis is arbitrary if a potential full search of the axis is acceptable
    pub fn experimental_find_within_index_range(
        &self,
        nearest_to: &Vector,
        negative_meters: &f64,
        positive_meters: &f64,
        axis: &Axis,
    ) -> Candidates {
        let nearest_to_index_vector = IndexVector::from_vector(nearest_to);
        let search_mode_range_index = &SearchMode::IndexRange(IndexRange::range_from_point(axis, &max_f64(negative_meters, positive_meters), negative_meters, positive_meters, nearest_to));
        let mut candidates: Candidates = BTreeMap::new();
        match axis {
            Axis::X => {
                let x_dynamic_search_order = DynamicSearchValidated::new(&Axis::X, nearest_to, nearest_to_index_vector.x, search_mode_range_index);
                x_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Y => {
                let y_dynamic_search_order = DynamicSearchValidated::new(&Axis::Y, nearest_to, nearest_to_index_vector.y, search_mode_range_index);
                y_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Z => {
                let z_dynamic_search_order = DynamicSearchValidated::new(&Axis::Z, nearest_to, nearest_to_index_vector.z, search_mode_range_index);
                z_dynamic_search_order.run(self, &mut candidates);
            },
        }
        candidates
    }

    //the axis chosen shouldn't actually matter, at this point, I believe the chosen axis is arbitrary if a potential full search of the axis is acceptable
    pub fn experimental_find_within_range(
        &self,
        nearest_to: &Vector,
        negative_meters: &f64, positive_meters: &f64,
        axis: &Axis,
    ) -> Candidates {
        //THIS MIGHT BREAK
        let starting_index = coordinate_to_index(match axis {
            Axis::X => &nearest_to.x,
            Axis::Y => &nearest_to.x,
            Axis::Z => &nearest_to.x,
        }, axis);
        self.experimental_find_within_index_range(nearest_to, negative_meters, positive_meters, axis)
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
        println!("Found {} candidates.", near_candidates.len());
        for (cumulative_distance, coordinate) in near_candidates {
            println!(
                "{}: d: {:18}, X: {}, Y: {}, Z: {}",
                start_time.elapsed().as_micros(),
                cumulative_distance,
                coordinate.x(),
                coordinate.y(),
                coordinate.z()
            );
        }

        for _ in 0..1 {
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
            let ordered_candidates_experimental = self.experimental_find_nearest(&random_vector, &Axis::X);
            //let ordered_candidates_from_index_range = self.experimental_find_within_index_range(&random_vector, (500, 500), &Axis::X);
            let ordered_candidates_from_range = self.experimental_find_within_range(&random_vector, &100.0, &100.0, &Axis::X);
            println!("Found {} candidates.", ordered_candidates.len());
            println!("Found {} candidates.", ordered_candidates_experimental.len());
            //println!("Found {} candidates.", ordered_candidates_from_index_range.len());
            println!("Found {} candidates.", ordered_candidates_from_range.len());
            
            assert_eq!(ordered_candidates.len(), ordered_candidates_experimental.len());

            /* for (direct_distance, reference_vector) in ordered_candidates_experimental/* ordered_candidates_from_index_range *//* ordered_candidates_from_range */ {
                println!(
                    "{}:I: d: {:18}, X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    direct_distance,
                    reference_vector.x(),
                    reference_vector.y(),
                    reference_vector.z()
                );
            } */

            println!(
                "{}: Nearest to random value: X: {}, Y: {}, Z: {}",
                start_time.elapsed().as_micros(),
                random_vector.x,
                random_vector.y,
                random_vector.z
            );

            let zero: &OrderedFloat<f64> = &OrderedFloat(0.0);
            let mut last: &OrderedFloat<f64> = &OrderedFloat(0.0);
            for (direct_distance, reference_vector) in ordered_candidates.iter() {
                /* println!(
                    "{}:3: d: {:18}, X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    direct_distance,
                    reference_vector.x(),
                    reference_vector.y(),
                    reference_vector.z()
                ); */
                assert!(direct_distance > last);
                last = direct_distance;
            }

            last = zero;
            for (direct_distance, reference_vector) in ordered_candidates_experimental.iter() {
                /* println!(
                    "{}:1: d: {:18}, X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    direct_distance,
                    reference_vector.x(),
                    reference_vector.y(),
                    reference_vector.z()
                ); */
                assert!(direct_distance > last);
                last = direct_distance;
            }

            /* last = zero;
            for (direct_distance, reference_vector) in ordered_candidates_from_index_range.iter() {
                println!(
                    "{}:I: d: {:18}, X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    direct_distance,
                    reference_vector.x(),
                    reference_vector.y(),
                    reference_vector.z()
                );
                assert!(direct_distance > last);
                last = direct_distance;
            } */
            
            last = zero;
            for (direct_distance, reference_vector) in ordered_candidates_from_range.iter() {
                /* println!(
                    "{}:R: d: {:18}, X: {}, Y: {}, Z: {}",
                    start_time.elapsed().as_micros(),
                    direct_distance,
                    reference_vector.x(),
                    reference_vector.y(),
                    reference_vector.z()
                ); */
                assert!(direct_distance > last);
                last = direct_distance;
            }
        }

        start_time.elapsed().as_millis()
    }
}
