#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::geographic_array::Axis;
    use crate::{
        geographic_array::GeographicArray,
        normalised_coordinate_to_index,
    };

    use crate::{Vector, MAX_RADIUS_METERS_X, MAX_RADIUS_METERS_Y, MAX_RADIUS_METERS_Z, normalise_negative_one_to_one_x, normalise_negative_one_to_one_y, normalise_negative_one_to_one_z, normalise_zero_to_one_x, normalise_zero_to_one_y, normalise_zero_to_one_z};

    #[test]
    fn test_normalise_negative_one_to_one() {
        assert_eq!(
            normalise_negative_one_to_one_x(23266.494456592045),
            0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_x(-23266.494456592045),
            -0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_y(23266.494456592045),
            0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_y(-23266.494456592045),
            -0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_z(23266.494456592045),
            0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one_z(-23266.494456592045),
            -0.3550185311369636,
        );
    }

    #[test]
    fn test_normalise_zero_to_one() {
        assert_eq!(
            normalise_zero_to_one_x(23266.494456592045),
            0.6775092655684818,
        );
        assert_eq!(
            normalise_zero_to_one_x(-23266.494456592045),
            0.3224907344315182,
        );
        assert_eq!(
            normalise_zero_to_one_y(23266.494456592045),
            0.6775092655684818,
        );
        assert_eq!(
            normalise_zero_to_one_y(-23266.494456592045),
            0.3224907344315182,
        );
        assert_eq!(
            normalise_zero_to_one_z(23266.494456592045),
            0.6775092655684818,
        );
        assert_eq!(
            normalise_zero_to_one_z(-23266.494456592045),
            0.3224907344315182,
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
            /* //normalise -1 to 1
            let coordinate_normalised_negative_one_to_one: f64 =
                normalise_negative_one_to_one(coordinate);
            assert_eq!(
                coordinate_normalised_negative_one_to_one,
                expected_negative_one_to_one
            ); */

            //normalise 0 to 1
            let coordinate_normalised_zero_to_one: f64 = match axis {
                Axis::X => normalise_zero_to_one_x(coordinate),
                Axis::Y => normalise_zero_to_one_y(coordinate),
                Axis::Z => normalise_zero_to_one_z(coordinate),
            };
            assert_eq!(coordinate_normalised_zero_to_one, expected_zero_to_one);

            let t: usize = normalised_coordinate_to_index(coordinate_normalised_zero_to_one);
            //panic!("{}", t);
            assert_eq!(t, expected_index_usize);

            //index after floor division
            let index_usize: usize = normalised_coordinate_to_index(coordinate_normalised_zero_to_one);
            assert_eq!(index_usize, expected_index_usize);
        }
    }

    #[test]
    fn test_fill_structure() {
        for _ in 0..1 {
            let mut geographic_array = GeographicArray::default();
            for _ in 0..1000000 {
                geographic_array.insert(Vector::generate_random());
            }
        }
    }
}
