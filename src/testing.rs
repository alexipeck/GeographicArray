#[cfg(test)]
mod tests {
    use crate::{MAX_RADIUS_METERS, CAPACITY_USIZE, normalise_zero_to_one, CAPACITY_F64};

    use {
        crate::{
            normalise_negative_one_to_one,
        },
    };

    #[test]
    fn test_normalise_negative_one_to_one() {
        assert_eq!(
            normalise_negative_one_to_one(23266.494456592045),
            0.3550185311369636,
        );
        assert_eq!(
            normalise_negative_one_to_one(-23266.494456592045),
            -0.3550185311369636,
        );
    }

    #[test]
    fn test_index() {
        let mut test_values: Vec<(f64, f64, f64, f64, usize)> = Vec::new();
        test_values.push((48719.51797980408, 0.7434008480805065, 0.8717004240402533, 914044.1438384326, 914044));
        test_values.push((-23915.320550257253, -0.36491883163844685, 0.3175405841807766, 332965.435597942, 332965));
        test_values.push((17861.636053173745, 0.2725469368465232, 0.6362734684232616, 667181.08842539, 667181));
        test_values.push((MAX_RADIUS_METERS, 1.0, 1.0, 1048576.0, 1048576));
        test_values.push((-MAX_RADIUS_METERS, -1.0, 0.0, 0.0, 0));
        
        for (coordinate, expected_negative_one_to_one, expected_zero_to_one, expected_index_f64, expected_index_usize) in test_values {
            //normalise -1 to 1
            let coordinate_normalised_negative_one_to_one: f64 = normalise_negative_one_to_one(coordinate);
            assert_eq!(coordinate_normalised_negative_one_to_one, expected_negative_one_to_one);

            //normalise 0 to 1
            let coordinate_normalised_zero_to_one: f64 = normalise_zero_to_one(coordinate);
            assert_eq!(coordinate_normalised_zero_to_one, expected_zero_to_one);

            //index before flood division
            let index_f64: f64 = CAPACITY_F64 * coordinate_normalised_zero_to_one;
            assert_eq!(index_f64, expected_index_f64);

            //index after floor division
            let index_usize: usize = index_f64 as usize;
            assert_eq!(index_usize, expected_index_usize);
        }
    }
}