mod parser;
pub mod scene_builder;

use core::Camera;
use self::scene_builder::SceneBuilder;

pub fn parse(string: &str) -> Camera {
    let mut builder = SceneBuilder::new();
    parser::parse_SceneFile(&mut builder, string).unwrap();
    builder.build_camera()
}
