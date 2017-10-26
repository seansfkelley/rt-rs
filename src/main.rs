#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    debug!("debug");
    info!("info");
    warn!("warn");

    println!("Hello, world!");
}
