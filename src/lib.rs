use {
    std::{rc::Rc, ops::Deref},
};

pub mod testing;
pub mod geographic_array;

pub const MAX_RADIUS_METERS: f64 = 65536.0;
const CUMULATIVE_DISTANCE_THRESHOLD: f64 = 10000.0;//within 10km cumulatively (x + y + z)

//Must be even, must be base 2
pub const ZONES_USIZE: usize = 1048576;//Actual value to edit
pub const ZONES_F64: f64 = ZONES_USIZE as f64;

#[derive(Clone, PartialEq, Debug)]
pub struct Vector {
    x: T,
    y: T,
    z: T,
}

impl Vector {
    pub fn to_real(&self) -> Self {
        let mut clone = self.clone();
        clone.make_real();
        clone
    }

    pub fn make_real(&mut self) {
        if let T::Reference(x) = self.x.clone() {
            self.x = T::Real(*x.deref());
        }
        if let T::Reference(y) = self.y.clone() {
            self.y = T::Real(*y.deref());
        }
        if let T::Reference(z) = self.z.clone() {
            self.z = T::Real(*z.deref());
        }
    }

    pub fn new_real_x(x: f64, y: Rc<f64>, z: Rc<f64>) -> Self {
        Self {
            x: T::Real(x),
            y: T::Reference(y),
            z: T::Reference(z),
        }
    }

    pub fn new_real_y(x: Rc<f64>, y: f64, z: Rc<f64>) -> Self {
        Self {
            x: T::Reference(x),
            y: T::Real(y),
            z: T::Reference(z),
        }
    }

    pub fn new_real_z(x: Rc<f64>, y: Rc<f64>, z: f64) -> Self {
        Self {
            x: T::Reference(x),
            y: T::Reference(y),
            z: T::Real(z),
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: T::Real(x),
            y: T::Real(y),
            z: T::Real(z),
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

    pub fn is_equal(&self, vector: &Vector) -> bool {
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
        let vector_x_ref = vector.x_as_ref();
        let vector_y_ref = vector.y_as_ref();
        let vector_z_ref = vector.z_as_ref();
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
pub enum T {
    Real(f64),
    Reference(Rc<f64>),
}

impl T {
    pub fn get_value(&self) -> f64 {
        match self {
            Self::Real(t) => *t,
            Self::Reference(t) => *t.as_ref(),
        }
    }

    pub fn get_value_as_ref(&self) -> &f64 {
        match self {
            Self::Real(t) => t,
            Self::Reference(t) => t.as_ref(),
        }
    }
}

pub fn normalise_zero_to_one(number: f64) -> f64 {
    (number - -MAX_RADIUS_METERS) / (MAX_RADIUS_METERS - -MAX_RADIUS_METERS)
}

pub fn normalise_negative_one_to_one(number: f64) -> f64 {
    2.0 * ((number - -MAX_RADIUS_METERS) / (MAX_RADIUS_METERS - -MAX_RADIUS_METERS)) - 1.0
}

pub fn coordinate_to_index(number: f64) -> usize {
    ((ZONES_F64 * normalise_zero_to_one(number)) - 1.0) as usize
}

pub fn normalised_coordinate_to_index(number: f64) -> usize {
    ((ZONES_F64 * number) - 1.0) as usize
}