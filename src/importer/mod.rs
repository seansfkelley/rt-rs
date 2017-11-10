mod parser;
pub mod scene_builder;

use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use core::*;
use self::scene_builder::SceneBuilder;

lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"#[^\n]*(\n|$)").unwrap();
}

fn read_file_contents(path: &Path) -> String {
    let mut contents: String = String::new();
    File::open(path)
        .expect("couldn't open file")
        .read_to_string(&mut contents)
        .expect("couldn't read file after opening");

    contents
}

fn strip_comments(s: String) -> String {
    COMMENT_REGEX.replace_all(s.as_str(), "$1").into_owned()
}

#[derive(Debug)]
pub struct SceneFile {
    pub camera: Camera,
    pub parameters: RenderParamaters,
    pub objects: Vec<SceneObject>,
}

pub fn parse(path: &Path) -> SceneFile {
    let mut builder = SceneBuilder::new();
    parser::parse_SceneFile(&mut builder, strip_comments(read_file_contents(path)).as_str()).unwrap();
    SceneFile {
        camera: builder.build_camera(),
        parameters: builder.build_render_parameters(),
        objects: builder.objects,
    }
}
