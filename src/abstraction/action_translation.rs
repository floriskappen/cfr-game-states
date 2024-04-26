use rand::Rng; // Import the Rng trait to use random number generation

pub fn pseudo_harmonic_mapping_randomized(x: f64, a: f64, b: f64) -> f64 {
    let f_ab_x = ((b - x) * (1.0 + a)) / ((b - a) * (1.0 + x));
    let mut rng = rand::thread_rng();
    let threshold: f64 = rng.gen(); // Generates a random float between 0.0 and 1.0

    if threshold <= f_ab_x {
        a
    } else {
        b
    }
}
