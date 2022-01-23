use {
    geographic_array::{
        ZONES_USIZE,
        geographic_array::GeographicArray,
    },
};


fn main() {
    let mut zones: usize = ZONES_USIZE;
    for _ in 1..4 {
        let mut geographic_array = GeographicArray::new(zones);
        geographic_array.run();
        zones *= 2;
    }
}
