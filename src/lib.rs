use {
    std::{rc::Rc, ops::Deref},
};

pub mod testing;

pub const MAX_RADIUS_METERS: f64 = 65536.0;

//Must be even, must be base 2
pub const CAPACITY_USIZE: usize = 1048576;//Actual value to edit
pub const CAPACITY_F64: f64 = CAPACITY_USIZE as f64;

#[derive(Clone)]
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



    pub fn x(&self) -> f64 {
        self.x.get_value()
    }
    
    pub fn y(&self) -> f64 {
        self.y.get_value()
    }
    
    pub fn z(&self) -> f64 {
        self.z.get_value()
    }
}

#[derive(Clone)]
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
}

pub fn normalise_zero_to_one(number: f64) -> f64 {
    //(1.0 - 0.0) / (MAX_RADIUS_METERS - -MAX_RADIUS_METERS) * (number - MAX_RADIUS_METERS) + 1.0
    /* (max'-min')/(max-min)*(value-max)+max' */
    (number - -MAX_RADIUS_METERS) / (MAX_RADIUS_METERS - -MAX_RADIUS_METERS)
}

pub fn normalise_negative_one_to_one(number: f64) -> f64 {
    2.0 * ((number - -MAX_RADIUS_METERS) / (MAX_RADIUS_METERS - -MAX_RADIUS_METERS)) - 1.0
}