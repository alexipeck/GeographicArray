use std::vec;
use std::{ops::Deref, rc::Rc, collections::BTreeMap, fmt};
use ordered_float::OrderedFloat;
use rand::{prelude::ThreadRng, Rng};

pub const MAX_RADIUS_METERS_X: f64 = 65536.0;
pub const MAX_RADIUS_METERS_Y: f64 = 65536.0;
pub const MAX_RADIUS_METERS_Z: f64 = 32768.0;
pub const CUMULATIVE_DISTANCE_THRESHOLD: f64 = 10000.0; //within 10km cumulatively (x + y + z)
pub const ZONES_USIZE: usize = 1048576; //Must be even, must be base 2 - Actual value to edit
pub const ZONES_INDEXED_USIZE: usize = ZONES_USIZE - 1;
pub const ZONES_F64: f64 = ZONES_USIZE as f64;

type Candidates = BTreeMap<OrderedFloat<f64>, ReferenceVector>;

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub enum AxisIndex {
    X(usize),
    Y(usize),
    Z(usize),
}

impl AxisIndex {
    pub fn new(axis: &Axis, index: usize) -> Self {
        match axis {
            Axis::X => {
                assert!(index <= ZONES_INDEXED_USIZE);
                Self::X(index)
            },
            Axis::Y => {
                assert!(index <= ZONES_INDEXED_USIZE);
                Self::Y(index)
            },
            Axis::Z => {
                assert!(index <= ZONES_INDEXED_USIZE);
                Self::Z(index)
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IndexRange {
    axis: Axis,
    starting_index: usize,
    range_lower: usize,
    range_upper: usize,
    distance_threshold: f64,

    validate_by_radius: bool,
}

impl IndexRange {
    //the range and starting point that will be created by this function is likely to have a starting point other than the middle if it is near any edge of the universe
    pub fn radius_from_point_meters(axis: &Axis, radius_meters: &f64, starting_point: &Vector) -> Self {
        let mut lower: f64 = -radius_meters;
        let mut upper: f64 = *radius_meters;
        let starting_index: usize = normalised_coordinate_to_index(&match axis {
            Axis::X => {
                lower += starting_point.x;
                upper -= starting_point.x;
                if lower > MAX_RADIUS_METERS_X {
                    lower = MAX_RADIUS_METERS_X;
                } else if lower < -MAX_RADIUS_METERS_X {
                    lower = -MAX_RADIUS_METERS_X;
                }
                if upper > MAX_RADIUS_METERS_X {
                    upper = MAX_RADIUS_METERS_X;
                } else if upper < -MAX_RADIUS_METERS_X {
                    upper = -MAX_RADIUS_METERS_X;
                }
                normalise_zero_to_one_x(&starting_point.x)
            },
            Axis::Y => {
                lower += starting_point.y;
                upper -= starting_point.y;
                if lower > MAX_RADIUS_METERS_Y {
                    lower = MAX_RADIUS_METERS_Y;
                } else if lower < -MAX_RADIUS_METERS_Y {
                    lower = -MAX_RADIUS_METERS_Y;
                }
                if upper > MAX_RADIUS_METERS_Y {
                    upper = MAX_RADIUS_METERS_Y;
                } else if upper < -MAX_RADIUS_METERS_Y {
                    upper = -MAX_RADIUS_METERS_Y;
                }
                normalise_zero_to_one_y(&starting_point.y)
            },
            Axis::Z => {
                lower += starting_point.z;
                upper -= starting_point.z;
                if lower > MAX_RADIUS_METERS_Z {
                    lower = MAX_RADIUS_METERS_Z;
                } else if lower < -MAX_RADIUS_METERS_Z {
                    lower = -MAX_RADIUS_METERS_Z;
                }
                if upper > MAX_RADIUS_METERS_Z {
                    upper = MAX_RADIUS_METERS_Z;
                } else if upper < -MAX_RADIUS_METERS_Z {
                    upper = -MAX_RADIUS_METERS_Z;
                }
                normalise_zero_to_one_z(&starting_point.z)
            },
        });

        assert!(starting_index <= ZONES_INDEXED_USIZE);
        assert!(lower <= 1.0);
        assert!(upper <= 1.0);
        assert!(lower >= -1.0);
        assert!(upper >= -1.0);
        Self {
            axis: *axis,
            starting_index,
            range_lower: normalised_coordinate_to_index(&lower),
            range_upper: normalised_coordinate_to_index(&upper),
            distance_threshold: *radius_meters,
            validate_by_radius: false,
        }
    }

    //distance threshold required because the negative and positive meters parameters only define search area, not evaluation of an entity
    pub fn range_from_point(axis: &Axis, distance_threshold: &f64, negative_meters: &f64, positive_meters: &f64, starting_point: &Vector, validate_by_radius: bool) -> Self {
        let mut lower: f64 = -negative_meters;
        let mut upper: f64 = *positive_meters;
        let starting_index: usize = normalised_coordinate_to_index(&normalise_zero_to_one(axis, match axis {
            Axis::X => {
                lower += starting_point.x;
                upper += starting_point.x;
                if lower > MAX_RADIUS_METERS_X {
                    lower = MAX_RADIUS_METERS_X;
                } else if lower < -MAX_RADIUS_METERS_X {
                    lower = -MAX_RADIUS_METERS_X;
                }
                if upper > MAX_RADIUS_METERS_X {
                    upper = MAX_RADIUS_METERS_X;
                } else if upper < -MAX_RADIUS_METERS_X {
                    upper = -MAX_RADIUS_METERS_X;
                }
                println!("Starting point X f64: {}", starting_point.x);
                &starting_point.x
            },
            Axis::Y => {
                lower += starting_point.y;
                upper += starting_point.y;
                if lower > MAX_RADIUS_METERS_Y {
                    lower = MAX_RADIUS_METERS_Y;
                } else if lower < -MAX_RADIUS_METERS_Y {
                    lower = -MAX_RADIUS_METERS_Y;
                }
                if upper > MAX_RADIUS_METERS_Y {
                    upper = MAX_RADIUS_METERS_Y;
                } else if upper < -MAX_RADIUS_METERS_Y {
                    upper = -MAX_RADIUS_METERS_Y;
                }
                &starting_point.y
            },
            Axis::Z => {
                lower += starting_point.z;
                upper += starting_point.z;
                if lower > MAX_RADIUS_METERS_Z {
                    lower = MAX_RADIUS_METERS_Z;
                } else if lower < -MAX_RADIUS_METERS_Z {
                    lower = -MAX_RADIUS_METERS_Z;
                }
                if upper > MAX_RADIUS_METERS_Z {
                    upper = MAX_RADIUS_METERS_Z;
                } else if upper < -MAX_RADIUS_METERS_Z {
                    upper = -MAX_RADIUS_METERS_Z;
                }
                &starting_point.z
            },
        }));

        println!("Starting point X usize: {}", starting_index);
        
        assert!(starting_index <= ZONES_INDEXED_USIZE);
        Self {
            axis: *axis,
            starting_index,
            range_lower: coordinate_to_index(&lower, axis),
            range_upper: coordinate_to_index(&upper, axis),
            distance_threshold: *distance_threshold,
            validate_by_radius,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SearchMode {
    Nearest,
    All,
    IndexRange(IndexRange),
}

impl fmt::Display for SearchMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Nearest => write!(f, "Nearest"),
            Self::All => write!(f, "All"),
            Self::IndexRange(index_range) => write!(f, "IndexRange({:?})", index_range),
        }
    }
}

pub struct DynamicSearchValidated {
    //axis: Axis,
    coordinate: Vector,     //used for comparison only during work operation
    axis_index: AxisIndex,  //defines the work start position
    search_mode: SearchMode,//specifies how long to keep searching
}

impl DynamicSearchValidated {
    pub fn new(axis: &Axis, nearest_to: &Vector, index: usize, search_mode: &SearchMode) -> Self {
        Self {
            //axis: axis.clone(),
            coordinate: nearest_to.clone(),                      //validated when the vector is created, Vector::{new(), generate_random(), generate_random_seeded()}
            axis_index: AxisIndex::new(axis, index),             //validated in AxisIndex::new()    
            search_mode: search_mode.clone(),   
        }
    }

    //geographic_array is the structure that will be searched
    //candidates is a reference to the structure that good candidates will be stored in
    //further methods after the initial collection will be added that decide how searching will include or exclude items
    //order does matter
    pub fn run(&self, geographic_array: &GeographicArray, candidates: &mut Candidates) {
        fn remove(to_remove: &mut Vec<usize>, potential_candidates: &mut Vec<ReferenceVector>) {
            to_remove.reverse();
            for index in to_remove {
                potential_candidates.remove(*index);
            }
        }
        //it will be more efficient to sort through elements in a bag and exclude from there
        //this might become a pre-processing function
        fn invalidate_by_type(potential_candidates: &mut Vec<ReferenceVector>) {
            let mut to_remove: Vec<usize> = Vec::new();
            for (i, _potential_candidate) in potential_candidates.iter_mut().enumerate() {
                //Condition here
                if false {
                    to_remove.push(i);
                }
            }
            remove(&mut to_remove, potential_candidates);
        }

        //not great at all, will replace entirely, but this is cheap, but more for invalidation than validation
        fn _validate_by_cumulative_distance(coordinate: &Vector, potential_candidates: &mut Vec<ReferenceVector>, candidates: &mut Candidates) {
            let mut to_remove: Vec<usize> = Vec::new();
            for (i, reference_vector) in potential_candidates.iter().enumerate() {
                let cumulative_diff: f64 = reference_vector.calculate_cumulative_diff(coordinate);
                if cumulative_diff <= CUMULATIVE_DISTANCE_THRESHOLD {
                    candidates.insert(OrderedFloat(cumulative_diff), reference_vector.to_real());
                    to_remove.push(i);
                }
            }
            remove(&mut to_remove, potential_candidates);
        }

        fn _validate_by_distance_as_the_crow_flies_along_the_ground() {

        }

        //calculate the direct distance between two vectors
        fn validate_by_distance_as_the_crow_flies(coordinate: &Vector, potential_candidates: &mut Vec<ReferenceVector>, candidates: &mut Candidates, search_mode: &SearchMode) {
            let mut to_remove: Vec<usize> = Vec::new();
            for (i, reference_vector) in potential_candidates.iter().enumerate() {
                let distance: f64 = distance_between(&Vector::from_reference_vector(reference_vector), coordinate);
                match search_mode {
                    SearchMode::IndexRange(index_range) => {
                        if distance <= index_range.distance_threshold {
                            candidates.insert(OrderedFloat(distance), reference_vector.to_real());
                            to_remove.push(i);
                        } else {
                            if distance > 227023.36345 {
                                println!("{}:{}", distance, index_range.distance_threshold);
                            }
                            assert!(distance <= 227023.36345);
                        }
                    },
                    _ => break,
                }
            }
            remove(&mut to_remove, potential_candidates);
        }

        fn validate_all_potential_candidates(coordinate: &Vector, potential_candidates: &mut Vec<ReferenceVector>, candidates: &mut Candidates) {
            let mut to_remove: Vec<usize> = Vec::new();
            for (i, reference_vector) in potential_candidates.iter().enumerate() {
                let distance: f64 = distance_between(&Vector::from_reference_vector(reference_vector), coordinate);
                candidates.insert(OrderedFloat(distance), reference_vector.to_real());
                to_remove.push(i);
            }
            remove(&mut to_remove, potential_candidates);
        }

        let mut can_move_positive_next_iteration: bool = true;
        let mut can_move_negative_next_iteration: bool = false;
        let mut deviation_count = 0;
        while match self.search_mode {
            SearchMode::Nearest => {
                candidates.is_empty() && (can_move_negative_next_iteration || can_move_positive_next_iteration)
            },
            SearchMode::All | SearchMode::IndexRange(..) => {
                can_move_negative_next_iteration || can_move_positive_next_iteration
            },
            _ => panic!(),
        } {
            //local iteration scope candidates
            let mut potential_candidates: Vec<ReferenceVector> = Vec::new();

            //first index and index + deviation
            if can_move_positive_next_iteration {
                potential_candidates.append(&mut match self.axis_index {
                    AxisIndex::X(index) => geographic_array.x[index + deviation_count].clone(),
                    AxisIndex::Y(index) => geographic_array.y[index + deviation_count].clone(),
                    AxisIndex::Z(index) => geographic_array.z[index + deviation_count].clone(),
                });
            }

            //index - deviation
            if deviation_count > 0 && can_move_negative_next_iteration {
                potential_candidates.append(&mut match self.axis_index {
                    AxisIndex::X(index) => geographic_array.x[index - deviation_count].clone(),
                    AxisIndex::Y(index) => geographic_array.y[index - deviation_count].clone(),
                    AxisIndex::Z(index) => geographic_array.z[index - deviation_count].clone(),
                });
            }
            
            //invalidates elements by a non existant condition, removing them from the potential candidates
            //this is a blacklisting function, blacklisting tasks should be run first
            invalidate_by_type(&mut potential_candidates);
            
            //println!("Search Mode: {}", self.search_mode);
            match self.search_mode {
                SearchMode::IndexRange(index_range) => {
                    if index_range.validate_by_radius {
                        validate_by_distance_as_the_crow_flies(&self.coordinate, &mut potential_candidates, candidates, &self.search_mode);
                    } else {
                        validate_all_potential_candidates(&self.coordinate, &mut potential_candidates, candidates);
                    }
                },
                SearchMode::All | SearchMode::Nearest => {
                    validate_all_potential_candidates(&self.coordinate, &mut potential_candidates, candidates);
                },
            }
            //everything for the next iteration
            deviation_count += 1;
            
            match self.axis_index {
                AxisIndex::X(index) => {
                    can_move_negative_next_iteration = match self.search_mode {
                        SearchMode::IndexRange(index_range) => {
                            index as isize - deviation_count as isize >= index_range.range_lower as isize/*  && index as isize - deviation_count as isize >= negative as isize */
                        },
                        _ => {
                            index as isize - deviation_count as isize >= 0
                        },
                    };
                    can_move_positive_next_iteration = match self.search_mode {
                        SearchMode::IndexRange(index_range) => {
                            index as isize + deviation_count as isize <= index_range.range_upper as isize/*  && index as isize - deviation_count as isize >= negative as isize */
                        },
                        _ => {
                            index as isize + deviation_count as isize <= ZONES_INDEXED_USIZE as isize
                        },
                    };
                },
                AxisIndex::Y(index) => {
                    can_move_negative_next_iteration = match self.search_mode {
                        SearchMode::IndexRange(index_range) => {
                            index as isize - deviation_count as isize >= index_range.range_lower as isize/*  && index as isize - deviation_count as isize >= negative as isize */
                        },
                        _ => {
                            index as isize - deviation_count as isize >= 0
                        },
                    };
                    can_move_positive_next_iteration = match self.search_mode {
                        SearchMode::IndexRange(index_range) => {
                            index as isize + deviation_count as isize <= index_range.range_upper as isize/*  && index as isize - deviation_count as isize >= negative as isize */
                        },
                        _ => {
                            index as isize + deviation_count as isize <= ZONES_INDEXED_USIZE as isize
                        },
                    };
                },
                AxisIndex::Z(index) => {
                    can_move_negative_next_iteration = match self.search_mode {
                        SearchMode::IndexRange(index_range) => {
                            index as isize - deviation_count as isize >= index_range.range_lower as isize/*  && index as isize - deviation_count as isize >= negative as isize */
                        },
                        _ => {
                            index as isize - deviation_count as isize >= 0
                        },
                    };
                    can_move_positive_next_iteration = match self.search_mode {
                        SearchMode::IndexRange(index_range) => {
                            index as isize + deviation_count as isize <= index_range.range_upper as isize/*  && index as isize - deviation_count as isize >= negative as isize */
                        },
                        _ => {
                            index as isize + deviation_count as isize <= ZONES_INDEXED_USIZE as isize
                        },
                    };
                },
            };
        }
    }
}

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
            x: normalised_coordinate_to_index(&normalise_zero_to_one_x(&vector.x)),
            y: normalised_coordinate_to_index(&normalise_zero_to_one_y(&vector.y)),
            z: normalised_coordinate_to_index(&normalise_zero_to_one_z(&vector.z)),
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
        assert!(x >= -MAX_RADIUS_METERS_X);
        assert!(x <= MAX_RADIUS_METERS_X);
        assert!(y >= -MAX_RADIUS_METERS_Y);
        assert!(y <= MAX_RADIUS_METERS_Y);
        assert!(z >= -MAX_RADIUS_METERS_Z);
        assert!(z <= MAX_RADIUS_METERS_Z);
        Self { x, y, z }
    }

    pub fn from_reference_vector(reference_vector: &ReferenceVector) -> Self {
        Self {
            x: reference_vector.x(),
            y: reference_vector.y(),
            z: reference_vector.z(),
        }
    }

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

pub fn normalise_zero_to_one(axis: &Axis, number: &f64) -> f64 {
    match axis {
        Axis::X => (number - -MAX_RADIUS_METERS_X) / (MAX_RADIUS_METERS_X - -MAX_RADIUS_METERS_X),
        Axis::Y => (number - -MAX_RADIUS_METERS_Y) / (MAX_RADIUS_METERS_Y - -MAX_RADIUS_METERS_Y),
        Axis::Z => (number - -MAX_RADIUS_METERS_Z) / (MAX_RADIUS_METERS_Z - -MAX_RADIUS_METERS_Z),
    }
}

pub fn normalise_zero_to_one_x(number: &f64) -> f64 {
    (number - -MAX_RADIUS_METERS_X) / (MAX_RADIUS_METERS_X - -MAX_RADIUS_METERS_X)
}

pub fn normalise_zero_to_one_y(number: &f64) -> f64 {
    (number - -MAX_RADIUS_METERS_Y) / (MAX_RADIUS_METERS_Y - -MAX_RADIUS_METERS_Y)
}

pub fn normalise_zero_to_one_z(number: &f64) -> f64 {
    (number - -MAX_RADIUS_METERS_Z) / (MAX_RADIUS_METERS_Z - -MAX_RADIUS_METERS_Z)
}

pub fn vector_coordinate_to_index(vector: &Vector, axis: &Axis) -> usize {
    let index = normalised_coordinate_to_index(&match axis {
        Axis::X => normalise_zero_to_one_x(&vector.x),
        Axis::Y => normalise_zero_to_one_y(&vector.y),
        Axis::Z => normalise_zero_to_one_z(&vector.z),
    });
    assert!(index <= ZONES_INDEXED_USIZE);
    index
}

pub fn coordinate_to_index(number: &f64, axis: &Axis) -> usize {
    let index = normalised_coordinate_to_index(&match axis {
        Axis::X => normalise_zero_to_one_x(number),
        Axis::Y => normalise_zero_to_one_y(number),
        Axis::Z => normalise_zero_to_one_z(number),
    });
    assert!(index <= ZONES_INDEXED_USIZE);
    index
}

pub fn coordinate_to_index_x(number: &f64) -> usize {
    let index = normalised_coordinate_to_index(&normalise_zero_to_one_x(number));
    assert!(index <= ZONES_INDEXED_USIZE);
    index
}

pub fn coordinate_to_index_y(number: &f64) -> usize {
    let index = normalised_coordinate_to_index(&normalise_zero_to_one_y(number));
    assert!(index <= ZONES_INDEXED_USIZE);
    index
}

pub fn coordinate_to_index_z(number: &f64) -> usize {
    let index = normalised_coordinate_to_index(&normalise_zero_to_one_z(number));
    assert!(index <= ZONES_INDEXED_USIZE);
    index
}

//implied 0 to 1 normalisation
pub fn normalised_coordinate_to_index(number: &f64) -> usize {
    let index = ((ZONES_F64 * number) - 1.0) as usize;
    if index > ZONES_INDEXED_USIZE {
        println!("left: {}, right: {}", index, ZONES_INDEXED_USIZE);
    }
    assert!(index <= ZONES_INDEXED_USIZE);
    index
}

pub fn distance_between(one: &Vector, two: &Vector) -> f64 {
    (((two.x - one.x).powi(2)) + ((two.y - one.y).powi(2)) + ((two.z - one.z).powi(2))).sqrt()
}

pub fn max_f64(one: &f64, two: &f64) -> f64 {
    if one > two {
        return *one;
    } else {
        return *two;
    }
}

pub struct GeographicArray {
    pub x: Vec<Vec<ReferenceVector>>,
    pub y: Vec<Vec<ReferenceVector>>,
    pub z: Vec<Vec<ReferenceVector>>,
}

impl GeographicArray {

    //TODO: allow the zones value to be automatically generated based on the maximum expected density
    //NOTE: this is not a maximum, but lookup time will technically get slower once it gets beyond the expected density threshold
    pub fn new(zones: usize) -> Self {
        Self {
            x: vec![Vec::new(); zones],
            y: vec![Vec::new(); zones],
            z: vec![Vec::new(); zones],
        }
    }
    pub fn default() -> Self {
        Self {
            x: vec![Vec::new(); ZONES_USIZE],
            y: vec![Vec::new(); ZONES_USIZE],
            z: vec![Vec::new(); ZONES_USIZE],
        }
    }

    pub fn insert(&mut self, vector: Vector) -> IndexVector {
        let x_normalised_index: usize = coordinate_to_index_x(&vector.x);
        let y_normalised_index: usize = coordinate_to_index_y(&vector.y);
        let z_normalised_index: usize = coordinate_to_index_z(&vector.z);
        let x_ref = Rc::new(vector.x);
        let y_ref = Rc::new(vector.y);
        let z_ref = Rc::new(vector.z);
        self.x[x_normalised_index].push(ReferenceVector::new_real_x(
            vector.x,
            y_ref.clone(),
            z_ref.clone(),
        ));
        self.y[y_normalised_index].push(ReferenceVector::new_real_y(
            x_ref.clone(),
            vector.y,
            z_ref,
        ));
        self.z[z_normalised_index].push(ReferenceVector::new_real_z(
            x_ref,
            y_ref,
            vector.z,
        ));
        IndexVector::new(x_normalised_index, y_normalised_index, z_normalised_index)
    }

    //this function returns more than one value because the extra data it returns took no extra work to attain
    //There will be a function that only returns one value available
    //as well as an experimental version where you only need to search one axis
    pub fn find_nearest(
        &self,
        nearest_to: &Vector,
    ) -> Candidates {
        let x_axis: &Axis = &Axis::X;
        let y_axis: &Axis = &Axis::Y;
        let z_axis: &Axis = &Axis::Z;

        let nearest_to_index_vector = IndexVector::from_vector(nearest_to);
        let search_mode = &SearchMode::Nearest;
        let x_dynamic_search_order = DynamicSearchValidated::new(x_axis, nearest_to, nearest_to_index_vector.x, search_mode);
        let y_dynamic_search_order = DynamicSearchValidated::new(y_axis, nearest_to, nearest_to_index_vector.y, search_mode);
        let z_dynamic_search_order = DynamicSearchValidated::new(z_axis, nearest_to, nearest_to_index_vector.z, search_mode);
        let mut candidates: Candidates = BTreeMap::new();
        x_dynamic_search_order.run(self, &mut candidates);
        y_dynamic_search_order.run(self, &mut candidates);
        z_dynamic_search_order.run(self, &mut candidates);
        candidates
    }

    //the axis chosen shouldn't actually matter, at this point, I believe the chosen axis is arbitrary if a potential full search of the axis is acceptable
    pub fn experimental_find_nearest(
        &self,
        nearest_to: &Vector,
        axis: &Axis,
    ) -> Candidates {
        let nearest_to_index_vector = IndexVector::from_vector(nearest_to);
        let search_mode_nearest = &SearchMode::Nearest;
        let mut candidates: Candidates = BTreeMap::new();
        match axis {
            Axis::X => {
                let x_dynamic_search_order = DynamicSearchValidated::new(&Axis::X, nearest_to, nearest_to_index_vector.x, search_mode_nearest);
                x_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Y => {
                let y_dynamic_search_order = DynamicSearchValidated::new(&Axis::Y, nearest_to, nearest_to_index_vector.y, search_mode_nearest);
                y_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Z => {
                let z_dynamic_search_order = DynamicSearchValidated::new(&Axis::Z, nearest_to, nearest_to_index_vector.z, search_mode_nearest);
                z_dynamic_search_order.run(self, &mut candidates);
            },
        }
        candidates
    }

    //the axis chosen shouldn't actually matter, at this point, I believe the chosen axis is arbitrary if a potential full search of the axis is acceptable
    pub fn experimental_find_within_range(
        &self,
        nearest_to: &Vector,
        negative_meters: &f64,
        positive_meters: &f64,
        validate_by_radius: bool,
        axis: &Axis,
    ) -> Candidates {
        let nearest_to_index_vector = IndexVector::from_vector(nearest_to);
        let search_mode_range_index = &SearchMode::IndexRange(IndexRange::range_from_point(axis, &max_f64(&negative_meters.abs(), &positive_meters.abs()), negative_meters, positive_meters, nearest_to, validate_by_radius));
        println!("{:?}", search_mode_range_index);
        let mut candidates: Candidates = BTreeMap::new();
        match axis {
            Axis::X => {
                let x_dynamic_search_order = DynamicSearchValidated::new(&Axis::X, nearest_to, nearest_to_index_vector.x, search_mode_range_index);
                x_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Y => {
                let y_dynamic_search_order = DynamicSearchValidated::new(&Axis::Y, nearest_to, nearest_to_index_vector.y, search_mode_range_index);
                y_dynamic_search_order.run(self, &mut candidates);
            },
            Axis::Z => {
                let z_dynamic_search_order = DynamicSearchValidated::new(&Axis::Z, nearest_to, nearest_to_index_vector.z, search_mode_range_index);
                z_dynamic_search_order.run(self, &mut candidates);
            },
        }
        candidates
    }
}
