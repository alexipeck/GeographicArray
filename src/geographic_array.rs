use std::{vec, collections::HashSet};

use {
    std::rc::Rc,
    crate::{Vector, ZONES_USIZE, coordinate_to_index, ZONES_F64},
};

pub enum Axis {
    X,
    Y,
    Z,
}

/* pub struct XElement {

}

pub struct YElement {

}

pub struct ZElement {

} */

pub struct GeographicArray {
    zones: usize,
    pub x: Vec<Vec<Vector>>,
    //_x_median_index: usize,

    pub y: Vec<Vec<Vector>>,
    //_y_median_index: usize,

    pub z: Vec<Vec<Vector>>,
    //_z_median_index: usize,
}

impl GeographicArray {
    pub fn new(zones: usize) -> Self {
        Self {
            zones,
            x: vec![Vec::new(); zones],
            y: vec![Vec::new(); zones],
            z: vec![Vec::new(); zones],
        }
    }
    pub fn default() -> Self {
        Self {
            zones: ZONES_USIZE,
            x: vec![Vec::new(); ZONES_USIZE],
            y: vec![Vec::new(); ZONES_USIZE],
            z: vec![Vec::new(); ZONES_USIZE],
        }
    }
    
    //store the max value that the structure has had to push the data away from it's expected normalised index
    //use a hashset to store how far the index was pushed away from the expected index, using an f64 as the key
    //this above line could be considered a cache for lookups, this information doesn't need to persistent, though if it is, it might make for faster loading times
    //using some sort of value, similar to how a skip list works, it should be used to guess how far to jump, this should also be an automiatically tuneable value from the perspective of the data structure
    pub fn insert(&mut self, x: f64, y: f64, z: f64) {
        let x_normalised_index: usize = coordinate_to_index(x);
        let y_normalised_index: usize = coordinate_to_index(y);
        let z_normalised_index: usize = coordinate_to_index(z);

        let x_ref = Rc::new(x);
        let y_ref = Rc::new(y);
        let z_ref = Rc::new(z);
        let xx = Vector::new_real_x(x, y_ref.clone(), z_ref.clone());
        let yy = Vector::new_real_y(x_ref.clone(), y, z_ref.clone());
        let zz = Vector::new_real_z(x_ref.clone(), y_ref.clone(), z);
        //Best case insertion
        //Has no bounds checks
        self.x[x_normalised_index].push(xx);
        self.y[y_normalised_index].push(yy);
        self.z[z_normalised_index].push(zz);
        /* self.x[x_normalised_index] = Some(xx);
        let positive: bool;
        if xx.x() > self.x[x_normalised_index].as_ref().unwrap().x() {
            positive = true;
        } else {
            positive = false;
        }
        let mut index: usize = x_normalised_index;
        loop {
            if positive {
                if index != CAPACITY_USIZE - 1 {
                    index += 1;
                } else {
                    panic!("Index thingo is trying to go above it's index range, expected to enter at: {}", x_normalised_index);
                }
            } else {
                if index != 0 {
                    index -= 1;
                } else {
                    panic!("Index thingo is trying to go below it's index range, expected to enter at: {}", x_normalised_index);
                }
            }
            if self.x[index].is_none() {
                break;
            }
        }
        self.x[index] = Some(xx); */

        /* if self.y[y_normalised_index].is_none() {
            self.y[y_normalised_index] = Some(yy);
        } else {
            let positive: bool;
            if yy.y() > self.y[y_normalised_index].as_ref().unwrap().y() {
                positive = true;
            } else {
                positive = false;
            }
            let mut index: usize = y_normalised_index;
            loop {
                if positive {
                    if index != CAPACITY_USIZE - 1 {
                        index += 1;
                    } else {
                        panic!("Index thingo is trying to go above it's index range, expected to enter at: {}", y_normalised_index);
                    }
                } else {
                    if index != 0 {
                        index -= 1;
                    } else {
                        panic!("Index thingo is trying to go below it's index range, expected to enter at: {}", y_normalised_index);
                    }
                }
                if self.y[index].is_none() {
                    break;
                }
            }
            self.y[index] = Some(yy);
        }

        if self.z[z_normalised_index].is_none() {
            self.z[z_normalised_index] = Some(zz);
        } else {
            let positive: bool;
            if zz.z() > self.z[z_normalised_index].as_ref().unwrap().y() {
                positive = true;
            } else {
                positive = false;
            }
            let mut index: usize = z_normalised_index;
            loop {
                if positive {
                    if index != CAPACITY_USIZE - 1 {
                        index += 1;
                    } else {
                        panic!("Index thingo is trying to go above it's index range, expected to enter at: {}", z_normalised_index);
                    }
                } else {
                    if index != 0 {
                        index -= 1;
                    } else {
                        panic!("Index thingo is trying to go below it's index range, expected to enter at: {}", z_normalised_index);
                    }
                }
                if self.z[index].is_none() {
                    break;
                }
            }
            self.z[index] = Some(zz);
        } */
    }

    pub fn find_nearest(&self, /* axis: Axis,  */nearest_to: Vector) -> Vec<Vector> {
        let mut x: Vec<Vector> = Vec::new();
        let mut y: Vec<Vector> = Vec::new();
        let mut z: Vec<Vector> = Vec::new();
        for t in &self.x[coordinate_to_index(nearest_to.x())] {
            x.push(t.clone());
        }
        for t in &self.y[coordinate_to_index(nearest_to.y())] {
            y.push(t.clone());
        }
        for t in &self.z[coordinate_to_index(nearest_to.z())] {
            z.push(t.clone());
        }
        /* match axis {
            Axis::X => {
                for t in &self.x[coordinate_to_index(nearest_to.x())] {
                    x.push(t.clone());
                }
            },
            Axis::Y => {
                for t in &self.y[coordinate_to_index(nearest_to.y())] {
                    y.push(t.clone());
                }
            },
            Axis::Z => {
                for t in &self.z[coordinate_to_index(nearest_to.z())] {
                    z.push(t.clone());
                }
            },
        } */
        let mut t: Vec<Vector> = Vec::new();
        let mut u: HashSet<f64> = HashSet::new();
        for m in x.iter() {
            /*
            Loop through the x, y and z vectors, collect all values that are in all 3 vectors, remove duplicates and return
            */
        }

        x//temp
    }
}