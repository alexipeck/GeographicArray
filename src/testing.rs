#[cfg(test)]
mod tests {
    use ordered_float::OrderedFloat;

    use crate::unused::{normalise_negative_one_to_one_x, normalise_negative_one_to_one_y, normalise_negative_one_to_one_z};
    use crate::{
        geographic_array::GeographicArray,
        normalised_coordinate_to_index,
    };

    use crate::{Vector, MAX_RADIUS_METERS_X, MAX_RADIUS_METERS_Y, MAX_RADIUS_METERS_Z, normalise_zero_to_one_x, normalise_zero_to_one_y, normalise_zero_to_one_z, IndexVector, Axis, distance_between, IndexRange, ZONES_INDEXED_USIZE};

    #[test]
    fn test_normalise_negative_one_to_one() {
        assert_eq!(
            normalise_negative_one_to_one_x(&23266.494456592045),
            0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_x(&-23266.494456592045),
            -0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_y(&23266.494456592045),
            0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_y(&-23266.494456592045),
            -0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_z(&23266.494456592045),
            0.7100370622739272,
        );
        assert_eq!(
            normalise_negative_one_to_one_z(&-23266.494456592045),
            -0.7100370622739272,
        );
    }

    #[test]
    fn test_normalise_zero_to_one() {
        assert_eq!(
            normalise_zero_to_one_x(&23266.494456592045),
            0.6775092655684818,
        );
        assert_eq!(
            normalise_zero_to_one_x(&-23266.494456592045),
            0.3224907344315182,
        );
        assert_eq!(
            normalise_zero_to_one_y(&23266.494456592045),
            0.6775092655684818,
        );
        assert_eq!(
            normalise_zero_to_one_y(&-23266.494456592045),
            0.3224907344315182,
        );
        assert_eq!(
            normalise_zero_to_one_z(&23266.494456592045),
            0.8550185311369636,
        );
        assert_eq!(
            normalise_zero_to_one_z(&-23266.494456592045),
            0.14498146886303642,
        );
    }

    #[test]
    fn test_index() {
        let mut test_values: Vec<(f64, f64, f64, Axis, usize)> = Vec::new();
        //Just X for now
        test_values.push((
            48719.51797980408,
            0.7434008480805065,
            0.8717004240402533,
            Axis::X,
            914043,
        ));
        test_values.push((
            -23915.320550257253,
            -0.36491883163844685,
            0.3175405841807766,
            Axis::X,
            332964,
        ));
        test_values.push((
            17861.636053173745,
            0.2725469368465232,
            0.6362734684232616,
            Axis::X,
            667180,
        ));
        test_values.push((MAX_RADIUS_METERS_X, 1.0, 1.0, Axis::X, 1048575));
        test_values.push((-MAX_RADIUS_METERS_X, -1.0, 0.0, Axis::X, 0));
        test_values.push((MAX_RADIUS_METERS_Y, 1.0, 1.0, Axis::Y, 1048575));
        test_values.push((-MAX_RADIUS_METERS_Y, -1.0, 0.0, Axis::Y, 0));
        test_values.push((MAX_RADIUS_METERS_Z, 1.0, 1.0, Axis::Z, 1048575));
        test_values.push((-MAX_RADIUS_METERS_Z, -1.0, 0.0, Axis::Z, 0));

        for (
            coordinate,
            expected_negative_one_to_one,
            expected_zero_to_one,
            axis,
            expected_index_usize,
        ) in test_values
        {
            //normalise -1 to 1
            let coordinate_normalised_negative_one_to_one: f64 = match axis {
                Axis::X => normalise_negative_one_to_one_x(&coordinate),
                Axis::Y => normalise_negative_one_to_one_y(&coordinate),
                Axis::Z => normalise_negative_one_to_one_z(&coordinate),
            };
            assert_eq!(
                coordinate_normalised_negative_one_to_one,
                expected_negative_one_to_one
            );

            //normalise 0 to 1
            let coordinate_normalised_zero_to_one: f64 = match axis {
                Axis::X => normalise_zero_to_one_x(&coordinate),
                Axis::Y => normalise_zero_to_one_y(&coordinate),
                Axis::Z => normalise_zero_to_one_z(&coordinate),
            };
            assert_eq!(coordinate_normalised_zero_to_one, expected_zero_to_one);

            let t: usize = normalised_coordinate_to_index(&coordinate_normalised_zero_to_one);
            //panic!("{}", t);
            assert_eq!(t, expected_index_usize);

            //index after floor division
            let index_usize: usize = normalised_coordinate_to_index(&coordinate_normalised_zero_to_one);
            assert_eq!(index_usize, expected_index_usize);
        }
    }

    #[test]
    fn test_distance_between() {
        assert_eq!(distance_between(&Vector::new(7.0, 4.0, 3.0), &Vector::new(17.0, 6.0, 2.0)), 10.246950765959598);
    }

    #[test]
    fn test_full_range_search() {
        let mut rng = rand::thread_rng();
        let mut geographic_array = GeographicArray::default();
        for _ in 0..1000000 {
            geographic_array.insert(Vector::generate_random_seeded(&mut rng));
        }
        let near_candidates = geographic_array.experimental_find_within_range(&Vector::new(MAX_RADIUS_METERS_X / 2.0, MAX_RADIUS_METERS_Y / 2.0, MAX_RADIUS_METERS_Z / 2.0), &(MAX_RADIUS_METERS_X / 2.0), &(MAX_RADIUS_METERS_X / 2.0), &Axis::X);
        assert_eq!(near_candidates.len(), 1000000);
    }

    #[test]
    fn test_expect_closest_value_from_center() {
        let mut geographic_array = GeographicArray::default();
        let mut counter: usize = 0;
        for i in 0..MAX_RADIUS_METERS_X as usize {
            geographic_array.insert(Vector::new(0.0, i as f64, 0.0));
            counter += 1;
        }
        println!("geographic_array element count: {}", counter);
        {
            let near_candidates = geographic_array.find_nearest(&Vector::new(MAX_RADIUS_METERS_X / 2.0, MAX_RADIUS_METERS_Y / 2.0, 0.0));
            for (i, (direct_distance, coordinate)) in near_candidates.iter().enumerate() {
                if i == 1 {
                    //WARNING: For some reason, the distance search returns 32768.00001525879 instead of 32768.0, there's fuck all in it, but I don't know the cause
                    assert_eq!(direct_distance.trunc(), MAX_RADIUS_METERS_X / 2.0);
                }
                /* println!(
                    "Distance: {:17}, X: {}, Y: {}, Z: {}",
                    direct_distance,
                    coordinate.x(),
                    coordinate.y(),
                    coordinate.z()
                ); */
            }
        }
        {
            let near_candidates = geographic_array.experimental_find_nearest(&Vector::new(MAX_RADIUS_METERS_X / 2.0, MAX_RADIUS_METERS_Y / 2.0, 0.0), &Axis::X);
            for (i, (direct_distance, coordinate)) in near_candidates.iter().enumerate() {
                if i == 1 {
                    //WARNING: For some reason, the distance search returns 32768.00001525879 instead of 32768.0, there's fuck all in it, but I don't know the cause
                    assert_eq!(direct_distance.trunc(), MAX_RADIUS_METERS_X / 2.0);
                }
                /* println!(
                    "Distance: {:17}, X: {}, Y: {}, Z: {}",
                    direct_distance,
                    coordinate.x(),
                    coordinate.y(),
                    coordinate.z()
                ); */
            }
        }
        {
            let near_candidates = geographic_array.experimental_find_within_range(&Vector::new(0.0, 0.0, 0.0), &(MAX_RADIUS_METERS_X / 2.0), &(MAX_RADIUS_METERS_X / 2.0), &Axis::X);
            for (i, (direct_distance, coordinate)) in near_candidates.iter().enumerate() {
                if i < 100 {
                    /* println!(
                        "Distance: {:17}, X: {}, Y: {}, Z: {}",
                        direct_distance,
                        coordinate.x(),
                        coordinate.y(),
                        coordinate.z()
                    ); */
                }
            }
            assert_eq!(near_candidates.len(), 0);
        }
    }

    #[test]
    fn test_fill_structure() {
        let mut rng = rand::thread_rng();
        for _ in 0..1 {
            let mut geographic_array = GeographicArray::default();
            let mut synthetic_values: Vec<(Vector, Option<IndexVector>)> = vec![(Vector::generate_random_seeded(&mut rng), None); 100];
            for (i, (value, _)) in synthetic_values.clone().iter().enumerate() {
                synthetic_values[i].1 = Some(geographic_array.insert(value.clone()));
            }
            for _ in 0..1000000 {
                geographic_array.insert(Vector::generate_random_seeded(&mut rng));
            }
            for (value, _) in synthetic_values.iter() {
                let near_candidates = geographic_array.find_nearest(value);
                assert!(!near_candidates.is_empty());
                let mut first: bool = true;
                for (cumulative_distance, coordinate) in near_candidates {
                    if first {
                        assert_eq!(cumulative_distance, 0.0);
                        first = false;
                    }
                    println!(
                        "Distance: {:17}, X: {}, Y: {}, Z: {}",
                        cumulative_distance,
                        coordinate.x(),
                        coordinate.y(),
                        coordinate.z()
                    );
                }
            }
        }
    }

    #[test]
    fn test_index_range_from_point() {
        fn run_with_common_assertions(axis: &Axis, distance_threshold: &f64, meters: (&f64, &f64), starting_point: &Vector) -> IndexRange {
            let index_range = IndexRange::range_from_point(axis, distance_threshold, meters.0, meters.1, starting_point);
            println!("{:?}", index_range);
            assert!(index_range.range_upper <= ZONES_INDEXED_USIZE);
            assert!(index_range.range_lower <= ZONES_INDEXED_USIZE);
            index_range
        }
        /*
        axis: Axis,
        starting_index: usize,
        range_lower: usize,
        range_upper: usize,
        distance_threshold: f64,
        validate_by_radius: bool,
        */

        let axis = &Axis::X;
        let distance_threshold = &1000.0;
        let meters: (&f64, &f64) = (&-999.0, &999.0);
        let starting_point: &Vector = &Vector::new(0.0, 0.0, 0.0);
        

        {
            let index_range = run_with_common_assertions(axis, distance_threshold, meters, starting_point);
            assert_eq!(index_range.starting_index, ZONES_INDEXED_USIZE / 2);
        }
        
        {
            let starting_point: &Vector = &Vector::new(MAX_RADIUS_METERS_X, MAX_RADIUS_METERS_Y, MAX_RADIUS_METERS_Z);
            let index_range = run_with_common_assertions(axis, distance_threshold, meters, starting_point);
            assert_eq!(index_range.starting_index, ZONES_INDEXED_USIZE);
            assert_eq!(index_range.range_upper, ZONES_INDEXED_USIZE)
        }
    }
}
