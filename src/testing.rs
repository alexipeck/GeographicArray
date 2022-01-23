#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{MAX_RADIUS_METERS, normalise_zero_to_one, coordinate_to_index, normalised_coordinate_to_index, geographic_array::GeographicArray};

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
        let mut test_values: Vec<(f64, f64, f64, usize)> = Vec::new();
        test_values.push((48719.51797980408, 0.7434008480805065, 0.8717004240402533, 914043));
        test_values.push((-23915.320550257253, -0.36491883163844685, 0.3175405841807766, 332964));
        test_values.push((17861.636053173745, 0.2725469368465232, 0.6362734684232616, 667180));
        test_values.push((MAX_RADIUS_METERS, 1.0, 1.0, 1048575));
        test_values.push((-MAX_RADIUS_METERS, -1.0, 0.0, 0));
        
        for (coordinate, expected_negative_one_to_one, expected_zero_to_one, expected_index_usize) in test_values {
            //normalise -1 to 1
            let coordinate_normalised_negative_one_to_one: f64 = normalise_negative_one_to_one(coordinate);
            assert_eq!(coordinate_normalised_negative_one_to_one, expected_negative_one_to_one);

            //normalise 0 to 1
            let coordinate_normalised_zero_to_one: f64 = normalise_zero_to_one(coordinate);
            assert_eq!(coordinate_normalised_zero_to_one, expected_zero_to_one);

            let t: usize = normalised_coordinate_to_index(coordinate_normalised_zero_to_one);
            //panic!("{}", t);
            assert_eq!(t, expected_index_usize);
            
            //index after floor division
            let index_usize: usize = coordinate_to_index(coordinate);
            assert_eq!(index_usize, expected_index_usize);
        }
    }

    #[test]
    fn test_fill_structure() {
        for _ in 0..1 {
            let mut t = GeographicArray::default();
            for _ in 0..1000000 {
                let mut rng = rand::thread_rng();
                let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
                let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
                let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
                t.insert(x, y, z);
            }
        }
    }
}