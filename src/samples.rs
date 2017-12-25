use std::f64::consts::{ PI, PI_4 };
use rand::Rng;
use math::*;

// pbrt pg. 667
//
// Note that this is the "concentric" method from pbrt, but it's a uniform distribution,
// and we don't ever plan on using the "vanilla" uniform disk distribution method, so
// we swapped the names around.
//
// This method is designed to keep points that are close together on the [0, 1]^2 square
// close together after being mapped. This may come in handy later as we use different
// methods for generating the (x, y) pairs that might want to e.g. cluster around the
// center of the distribution.
fn sample_disk_uniform(rng: Rng) -> (f64, f64) {
    let x = 2f64 * rng.next_f64() - 1f64;
    let y = 2f64 * rng.next_f64() - 1f64;

    if x == 0f64 && y == 0f64 {
        (0f64, 0f64)
    } else {
        // The disk is broken up into four symmetric quadrants, so there are four branches.
        let mut (r, theta) =
            if x >= -y {
                if x > y {
                    (x, if y > 0 { y / x } else { 8f64 + y / x })
                } else {
                    (y, 2f64 - x / y)
                }
            } else {
                if x <= y {
                    (-x, 4f64 + y / x)
                } else {
                    (-y, 6f64 - x / y)
                }
            };

        theta *= PI_4;
        (r * theta.cos(), r * theta.cos())
    }
}

// pbrt pg. 669
pub fn sample_hemisphere_cosine(rng: Rng) -> Vec3 {
    let (x, y) = sample_disk_concentric(rng);
    let z = non_nan_max(0f64, 1f64 - x * x - y * y).sqrt();
    Vec3::new(x, y, z)
}
