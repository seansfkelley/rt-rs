#[macro_use]
extern crate log;
extern crate env_logger;

mod vector;

use vector::Vec3;

fn main() {
    env_logger::init().unwrap();

    debug!("debug");
    info!("info");
    warn!("warn");

    println!("Hello, world!");

    let normal = Vec3::new(1f64, 2f64, 3f64);
    let ray = Vec3::new(4f64, 5f64, 6f64);
    let thing = normal.cross(&ray);
    println!("normal: {:?}, ray: {:?}, result: {:?}", normal, ray, thing);
}
