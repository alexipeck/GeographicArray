use std::time::Instant;
use geographic_array::geographic_array::{ZONES_USIZE, Vector, GeographicArray, Axis};
use ordered_float::OrderedFloat;

fn main() {
    let mut zones: usize = ZONES_USIZE;
    //for _ in 1..4 {
        println!("Creating structure with {} zones on each axis.", zones);
        let mut geographic_array = GeographicArray::new(zones);


        let start_time = Instant::now();
        println!(
            "{}: Generating GeographicArray.",
            start_time.elapsed().as_micros()
        );
        let mut rng = rand::thread_rng();
        let synthetic_value: Vector = Vector::generate_random_seeded(&mut rng);
        geographic_array.insert(synthetic_value.clone());
        let values_to_insert: usize = 10000000;
        println!(
            "{}: Inserting a few values: {}",
            start_time.elapsed().as_micros(),
            values_to_insert,
        );
        for _ in 0..values_to_insert {
            geographic_array.insert(Vector::generate_random_seeded(&mut rng));
        }
        println!(
            "{}: Nearest to random synthetic value: X: {}, Y: {}, Z: {}",
            start_time.elapsed().as_micros(),
            synthetic_value.x,
            synthetic_value.y,
            synthetic_value.z,
        );

        //move to testing.rs
        let near_candidates = geographic_array.find_nearest(&synthetic_value);
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

            let ordered_candidates = geographic_array.find_nearest(&random_vector);
            let ordered_candidates_experimental = geographic_array.experimental_find_nearest(&random_vector, &Axis::X);
            //let ordered_candidates_from_index_range = geographic_array.experimental_find_within_index_range(&random_vector, (500, 500), &Axis::X);
            let ordered_candidates_from_range = geographic_array.experimental_find_within_range(&random_vector, &500.0, &500.0, true, &Axis::X);
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

        


        let execution_time = start_time.elapsed().as_millis();
        println!("Execution time was {}ms", execution_time);
        zones *= 2;
        println!("Deconstructing...");
        println!();
        println!();
    //}
}
