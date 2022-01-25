use std::{ops::Deref, rc::Rc};

use geographic_array::GeographicArray;
use rand::{prelude::ThreadRng, Rng};

pub mod geographic_array;
pub mod testing;

pub const MAX_RADIUS_METERS_X: f64 = 65536.0;
pub const MAX_RADIUS_METERS_Y: f64 = 65536.0;
pub const MAX_RADIUS_METERS_Z: f64 = 65536.0;

const CUMULATIVE_DISTANCE_THRESHOLD: f64 = 10000.0; //within 10km cumulatively (x + y + z)

//Must be even, must be base 2
pub const ZONES_USIZE: usize = 1048576; //Actual value to edit
pub const ZONES_INDEXED: usize = ZONES_USIZE - 1;
pub const ZONES_F64: f64 = ZONES_USIZE as f64;

#[derive(Clone, PartialEq, Debug)]
pub struct IndexVector {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl IndexVector {
    pub fn new(x:usize, y: usize, z: usize) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn from_vector(vector: &Vector) -> Self {
        Self {
            x: normalised_coordinate_to_index(normalise_zero_to_one_x(vector.x)),
            y: normalised_coordinate_to_index(normalise_zero_to_one_y(vector.y)),
            z: normalised_coordinate_to_index(normalise_zero_to_one_z(vector.z)),
        }
    }

    pub fn max_index(&self) -> usize {
        let maybe_largest: usize;
        if self.x > self.y {
            maybe_largest = self.x;
        } else {
            maybe_largest = self.y;
        }
        if maybe_largest > self.z {
            maybe_largest
        } else {
            self.z
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(-MAX_RADIUS_METERS_X..MAX_RADIUS_METERS_X);
        let y: f64 = rng.gen_range(-MAX_RADIUS_METERS_Y..MAX_RADIUS_METERS_Y);
        let z: f64 = rng.gen_range(-MAX_RADIUS_METERS_Z..MAX_RADIUS_METERS_Z);

        Self { x, y, z }
    }

    pub fn generate_random_seeded(rng: &mut ThreadRng) -> Self {
        let x: f64 = rng.gen_range(-MAX_RADIUS_METERS_X..MAX_RADIUS_METERS_X);
        let y: f64 = rng.gen_range(-MAX_RADIUS_METERS_Y..MAX_RADIUS_METERS_Y);
        let z: f64 = rng.gen_range(-MAX_RADIUS_METERS_Z..MAX_RADIUS_METERS_Z);

        Self { x, y, z }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReferenceVector {
    x: ValueType,
    y: ValueType,
    z: ValueType,
}

impl ReferenceVector {
    pub fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(-MAX_RADIUS_METERS_X..MAX_RADIUS_METERS_X);
        let y: f64 = rng.gen_range(-MAX_RADIUS_METERS_Y..MAX_RADIUS_METERS_Y);
        let z: f64 = rng.gen_range(-MAX_RADIUS_METERS_Z..MAX_RADIUS_METERS_Z);

        Self::new(x, y, z)
    }

    pub fn generate_random_seeded(rng: &mut ThreadRng) -> Self {
        let x: f64 = rng.gen_range(-MAX_RADIUS_METERS_X..MAX_RADIUS_METERS_X);
        let y: f64 = rng.gen_range(-MAX_RADIUS_METERS_Y..MAX_RADIUS_METERS_Y);
        let z: f64 = rng.gen_range(-MAX_RADIUS_METERS_Z..MAX_RADIUS_METERS_Z);

        Self::new(x, y, z)
    }

    pub fn to_real(&self) -> Self {
        let mut clone = self.clone();
        clone.make_real();
        clone
    }

    pub fn make_real(&mut self) {
        if let ValueType::Reference(x) = self.x.clone() {
            self.x = ValueType::Real(*x.deref());
        }
        if let ValueType::Reference(y) = self.y.clone() {
            self.y = ValueType::Real(*y.deref());
        }
        if let ValueType::Reference(z) = self.z.clone() {
            self.z = ValueType::Real(*z.deref());
        }
    }

    pub fn new_real_x(x: f64, y: Rc<f64>, z: Rc<f64>) -> Self {
        Self {
            x: ValueType::Real(x),
            y: ValueType::Reference(y),
            z: ValueType::Reference(z),
        }
    }

    pub fn new_real_y(x: Rc<f64>, y: f64, z: Rc<f64>) -> Self {
        Self {
            x: ValueType::Reference(x),
            y: ValueType::Real(y),
            z: ValueType::Reference(z),
        }
    }

    pub fn new_real_z(x: Rc<f64>, y: Rc<f64>, z: f64) -> Self {
        Self {
            x: ValueType::Reference(x),
            y: ValueType::Reference(y),
            z: ValueType::Real(z),
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: ValueType::Real(x),
            y: ValueType::Real(y),
            z: ValueType::Real(z),
        }
    }

    pub fn x(&self) -> f64 {
        self.x.get_value()
    }

    pub fn x_as_ref(&self) -> &f64 {
        self.x.get_value_as_ref()
    }

    pub fn y(&self) -> f64 {
        self.y.get_value()
    }

    pub fn y_as_ref(&self) -> &f64 {
        self.y.get_value_as_ref()
    }

    pub fn z(&self) -> f64 {
        self.z.get_value()
    }

    pub fn z_as_ref(&self) -> &f64 {
        self.z.get_value_as_ref()
    }

    pub fn is_equal(&self, vector: &ReferenceVector) -> bool {
        if self.x_as_ref() != vector.x_as_ref() {
            return false;
        }

        if self.y_as_ref() != vector.y_as_ref() {
            return false;
        }

        if self.z_as_ref() != vector.z_as_ref() {
            return false;
        }

        true
    }

    pub fn calculate_cumulative_diff(&self, vector: &Vector) -> f64 {
        let mut temp: f64 = 0.0;
        let self_x_ref = self.x_as_ref();
        let self_y_ref = self.y_as_ref();
        let self_z_ref = self.z_as_ref();
        let vector_x_ref = &vector.x;
        let vector_y_ref = &vector.y;
        let vector_z_ref = &vector.z;
        if self_x_ref > vector_x_ref {
            temp += self_x_ref - vector_x_ref;
        } else {
            temp += vector_x_ref - self_x_ref;
        }
        if self_y_ref > vector_y_ref {
            temp += self_y_ref - vector_y_ref;
        } else {
            temp += vector_y_ref - self_y_ref;
        }
        if self_z_ref > vector_z_ref {
            temp += self_z_ref - vector_z_ref;
        } else {
            temp += vector_z_ref - self_z_ref;
        }

        temp
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ValueType {
    Real(f64),
    Reference(Rc<f64>),
}

impl ValueType {
    pub fn get_value(&self) -> f64 {
        match self {
            Self::Real(value) => *value,
            Self::Reference(value) => *value.as_ref(),
        }
    }

    pub fn get_value_as_ref(&self) -> &f64 {
        match self {
            Self::Real(value) => value,
            Self::Reference(value) => value.as_ref(),
        }
    }
}


pub fn normalise_zero_to_one_x(number: f64) -> f64 {
    (number - -MAX_RADIUS_METERS_X) / (MAX_RADIUS_METERS_X - -MAX_RADIUS_METERS_X)
}

pub fn normalise_zero_to_one_y(number: f64) -> f64 {
    (number - -MAX_RADIUS_METERS_Y) / (MAX_RADIUS_METERS_Y - -MAX_RADIUS_METERS_Y)
}

pub fn normalise_zero_to_one_z(number: f64) -> f64 {
    (number - -MAX_RADIUS_METERS_Z) / (MAX_RADIUS_METERS_Z - -MAX_RADIUS_METERS_Z)
}


pub fn normalise_negative_one_to_one_x(number: f64) -> f64 {
    2.0 * ((number - -MAX_RADIUS_METERS_X) / (MAX_RADIUS_METERS_X - -MAX_RADIUS_METERS_X)) - 1.0
}

pub fn normalise_negative_one_to_one_y(number: f64) -> f64 {
    2.0 * ((number - -MAX_RADIUS_METERS_Y) / (MAX_RADIUS_METERS_Y - -MAX_RADIUS_METERS_Y)) - 1.0
}

pub fn normalise_negative_one_to_one_z(number: f64) -> f64 {
    2.0 * ((number - -MAX_RADIUS_METERS_Z) / (MAX_RADIUS_METERS_Z - -MAX_RADIUS_METERS_Z)) - 1.0
}

pub fn coordinate_to_index_x(number: f64) -> usize {
    normalised_coordinate_to_index(normalise_zero_to_one_x(number))
}

pub fn coordinate_to_index_y(number: f64) -> usize {
    normalised_coordinate_to_index(normalise_zero_to_one_y(number))
}

pub fn coordinate_to_index_z(number: f64) -> usize {
    normalised_coordinate_to_index(normalise_zero_to_one_z(number))
}

//implied 0 to 1 normalisation
pub fn normalised_coordinate_to_index(number: f64) -> usize {
    ((ZONES_F64 * number) - 1.0) as usize
}