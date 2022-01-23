use {
    geographic_array::{
        ZONES_USIZE,
        geographic_array::GeographicArray,
    },
};


fn main() {
    let mut zones: usize = ZONES_USIZE;
    for _ in 1..4 {
        println!("Creating structure with {} zones on each axis.", zones);
        let mut geographic_array = GeographicArray::new(zones);
        let execution_time = geographic_array.run();
        println!("Execution time was {}μs", execution_time);
        zones *= 2;
    }
}
