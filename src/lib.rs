pub mod geographic_array;

/* pub mod unused {
    use crate::{MAX_RADIUS_METERS_X, MAX_RADIUS_METERS_Y, MAX_RADIUS_METERS_Z};

    pub fn normalise_negative_one_to_one_x(number: &f64) -> f64 {
        2.0 * ((number - -MAX_RADIUS_METERS_X) / (MAX_RADIUS_METERS_X - -MAX_RADIUS_METERS_X)) - 1.0
    }
    
    pub fn normalise_negative_one_to_one_y(number: &f64) -> f64 {
        2.0 * ((number - -MAX_RADIUS_METERS_Y) / (MAX_RADIUS_METERS_Y - -MAX_RADIUS_METERS_Y)) - 1.0
    }
    
    pub fn normalise_negative_one_to_one_z(number: &f64) -> f64 {
        2.0 * ((number - -MAX_RADIUS_METERS_Z) / (MAX_RADIUS_METERS_Z - -MAX_RADIUS_METERS_Z)) - 1.0
    }
} */