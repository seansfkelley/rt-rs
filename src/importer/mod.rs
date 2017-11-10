mod parser;
pub mod scene_builder;

use core::*;
use self::scene_builder::SceneBuilder;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn read_file_contents(path: &Path) -> String {
    let mut contents: String = String::new();
    File::open(path)
        .expect("couldn't open file")
        .read_to_string(&mut contents)
        .expect("couldn't read file after opening");

    contents
}

#[derive(Debug)]
pub struct SceneFile {
    camera: Camera,
    parameters: RenderParamaters,
}

pub fn parse(path: &Path) -> SceneFile {
    let mut builder = SceneBuilder::new();
    parser::parse_SceneFile(&mut builder, read_file_contents(path).as_str()).unwrap();
    SceneFile {
        camera: builder.build_camera(),
        parameters: builder.build_render_parameters(),
    }
}
