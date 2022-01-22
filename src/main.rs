use geographic_array::{MAX_RADIUS_METERS, Vector, geographic_array::Axis, ZONES_USIZE};

use {
    geographic_array::geographic_array::GeographicArray,
    rand::Rng,
};


fn main() {
    let mut zones: usize = ZONES_USIZE;
    for _ in 0..3 {
        for i in 0..10 {
            let mut t = GeographicArray::new(zones);
            let mut rng = rand::thread_rng();
            for _ in 0..1000000 {
                let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
                let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
                let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
                t.insert(x, y, z);
            }
            let x: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let y: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            let z: f64 = rng.gen_range(-MAX_RADIUS_METERS..MAX_RADIUS_METERS);
            println!("Nearest to X: {}, Y: {}, Z: {}", x, y, z);
            let h = t.find_nearest(Vector::new(x, y, z));
            for g in h {
                println!("X: {}, Y: {}, Z: {}", g.x(), g.y(), g.z());
            }
            println!("Finished test iteration:  {}", i);
        }
        zones *= 2;
    }
    
    /* for (i, element) in t.x.iter().enumerate() {
        for t in element {
            println!("Index: {:10}, X: {}, Y: {}, Z: {}", i, t.x(), t.y(), t.z());
        }
    } */
}
