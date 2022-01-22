use {
    ::GeographicArray::{
        CAPACITY_F64,
        CAPACITY_USIZE,
        MAX_RADIUS_METERS,
        Vector,
        normalise_negative_one_to_one,
        normalise_zero_to_one
    },
    rand::Rng,
    std::rc::Rc,
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
    x: Vec<Option<Vector>>,
    //_x_median_index: usize,

    y: Vec<Option<Vector>>,
    //_y_median_index: usize,

    z: Vec<Option<Vector>>,
    //_z_median_index: usize,

    _capacity_split: Vec<usize>,
}

impl GeographicArray {
    pub fn default() -> Self {
        let mut _capacity_split = Vec::new();
        {
            let mut current_value: usize = CAPACITY_USIZE / 2;
            while current_value > 2 {
                _capacity_split.push(current_value);
                current_value /= 2;
            }
        }
        Self {
            x: vec![None; CAPACITY_USIZE],
            //_x_median_index: _capacity_split[0],

            y: vec![None; CAPACITY_USIZE],
            //_y_median_index: _capacity_split[0],

            z: vec![None; CAPACITY_USIZE],
            //_z_median_index: _capacity_split[0],

            _capacity_split,
        }
    }
    
    //store the max value that the structure has had to push the data away from it's expected normalised index
    //use a hashset to store how far the index was pushed away from the expected index, using an f64 as the key
    //this above line could be considered a cache for lookups, this information doesn't need to persistent, though if it is, it might make for faster loading times
    //using some sort of value, similar to how a skip list works, it should be used to guess how far to jump, this should also be an automiatically tuneable value from the perspective of the data structure
    pub fn insert(&mut self, x: f64, y: f64, z: f64) {
        let x_normalised_index: usize = (CAPACITY_F64 * normalise_zero_to_one(x)) as usize;
        let y_normalised_index: usize = (CAPACITY_F64 * normalise_zero_to_one(y)) as usize;
        let z_normalised_index: usize = (CAPACITY_F64 * normalise_zero_to_one(z)) as usize;

        let x_ref = Rc::new(x);
        let y_ref = Rc::new(y);
        let z_ref = Rc::new(z);
        let xx = Vector::new_real_x(x, y_ref.clone(), z_ref.clone());
        let yy = Vector::new_real_y(x_ref.clone(), y, z_ref.clone());
        let zz = Vector::new_real_z(x_ref.clone(), y_ref.clone(), z);
        //Best case insertion
        //Has no bounds checks
        if self.x[x_normalised_index].is_none() {
            self.x[x_normalised_index] = Some(xx);
        } else {
            if xx.x() > self.x[x_normalised_index].as_ref().unwrap().x() {
                if self.x[x_normalised_index + 1].is_none() {
                    self.x[x_normalised_index + 1] = Some(xx);
                } else {
                    panic!("Will panic until more than one retry has been implemented.");
                }
            } else {
                if self.x[x_normalised_index - 1].is_none() {
                    self.x[x_normalised_index - 1] = Some(xx);
                } else {
                    panic!("Will panic until more than one retry has been implemented.");
                }
            }
        }
        if self.y[y_normalised_index].is_none() {
            self.y[y_normalised_index] = Some(yy);
        } else {
            if yy.y() > self.y[y_normalised_index].as_ref().unwrap().y() {
                if self.y[y_normalised_index + 1].is_none() {
                    self.y[y_normalised_index + 1] = Some(yy);
                } else {
                    panic!("Will panic until more than one retry has been implemented.");
                }
            } else {
                if self.y[y_normalised_index - 1].is_none() {
                    self.y[y_normalised_index - 1] = Some(yy);
                } else {
                    panic!("Will panic until more than one retry has been implemented.");
                }
            }
        }
        if self.z[z_normalised_index].is_none() {
            self.z[z_normalised_index] = Some(zz);
        } else {
            if zz.z() > self.z[z_normalised_index].as_ref().unwrap().z() {
                if self.z[z_normalised_index + 1].is_none() {
                    self.z[z_normalised_index + 1] = Some(zz);
                } else {
                    panic!("Will panic until more than one retry has been implemented.");
                }
            } else {
                if self.z[z_normalised_index - 1].is_none() {
                    self.z[z_normalised_index - 1] = Some(zz);
                } else {
                    panic!("Will panic until more than one retry has been implemented.");
                }
            }
        }
    }

    pub fn find_nearest(&self, axis: Axis, nearest_to: Vector) {
        let mut nearest_value: Option<Vector> = None;
        match axis {
            Axis::X => {
                let mut positive: bool = true;
                let x_normalised: f64 = normalise_negative_one_to_one(nearest_to.x());
                let guess_index: usize = (CAPACITY_F64 * x_normalised) as usize;

                if let Some(element) = &self.x[guess_index] {
                    if element.x() == nearest_to.x() {
                        println!("Value found");
                        nearest_value = Some(element.to_real());
                    } else {
                        println!("First guess did not find value");
                        panic!();
                    }
                } else {
                    //keep looking, panic for now
                    panic!();
                }
                /*
                binary search, if the current value is less than the one I want the nearest value to, bring the search to the next granularity
                if more, than it, drop by the next level or granularity
                */
                /* let current_granularity_index: usize = 0;
                let current_narrowed_index: usize = self._capacity_split[0];
                for granularity in self._capacity_split {
                    if nearest_to.0 > self.x[granularity].0 {
                        
                    }
                } */
            },
            Axis::Y => {
                let mut positive: bool = true;
                let y_normalised: f64 = normalise_negative_one_to_one(nearest_to.y());
                let guess_index: usize = (CAPACITY_F64 * y_normalised) as usize;

                if let Some(element) = &self.y[guess_index] {
                    if element.y() == nearest_to.y() {
                        println!("Value found");
                        nearest_value = Some(element.to_real());
                    } else {
                        println!("First guess did not find value");
                        panic!();
                    }
                } else {
                    //keep looking, panic for now
                    panic!();
                }
            },
            Axis::Z => {
                let mut positive: bool = true;
                let z_normalised: f64 = normalise_negative_one_to_one(nearest_to.z());
                let guess_index: usize = (CAPACITY_F64 * z_normalised) as usize;

                if let Some(element) = &self.z[guess_index] {
                    if element.z() == nearest_to.z() {
                        println!("Value found");
                        nearest_value = Some(element.to_real());
                    } else {
                        println!("First guess did not find value");
                        panic!();
                    }
                } else {
                    //keep looking, panic for now
                    panic!();
                }
            },
        }
        if let Some(nearest_value) = nearest_value {
            println!("X: {}, Y: {}, Z: {}", nearest_value.x(), nearest_value.x(), nearest_value.z());
        }
    }
}


fn main() {
    let mut t = GeographicArray::default();
    for _ in 0..10000 {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
        t.insert(x, y, z);
    }
    for (i, element) in t.x.iter().enumerate() {
        if let Some(t) = element {
            println!("Index: {:10}, X: {}, Y: {}, Z: {}", i, t.x(), t.y(), t.z());
        }
    }
}
