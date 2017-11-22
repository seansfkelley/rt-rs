mod parser;
pub mod scene_builder;

use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use core::*;
use lalrpop_util::ParseError;
use self::scene_builder::SceneBuilder;

lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"#[^\n]*(\n|$)").unwrap();
    static ref NEWLINE_REGEX: Regex = Regex::new(r"\n").unwrap();
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
    // It is VERY IMPORTANT that this regex does not drop newlines, else our line counts will be off.
    COMMENT_REGEX.replace_all(s.as_str(), "$1").into_owned()
}

#[derive(Debug)]
pub struct SceneFile {
    pub camera: Box<Camera>,
    pub animation: (u32, Transform),
    pub parameters: RenderParamaters,
    pub objects: Vec<SceneObject>,
    pub lights: Vec<Light>,
}

pub fn parse(path: &Path) -> SceneFile {
    let mut builder = SceneBuilder::new();
    let file_source = strip_comments(read_file_contents(path));
    let line_lengths: Vec<usize> = NEWLINE_REGEX
        .split(file_source.as_str())
        .map(|text| text.len())
        .collect();

    let get_line_and_column = |i: usize|  {
        let mut line = 0usize;
        let mut index = i;
        while line < line_lengths.len() && index >= (line_lengths[line] + 1) {
            index -= line_lengths[line] + 1;
            line += 1;
        }
        (line + 1, index + 1)
    };

    match parser::parse_SceneFile(&mut builder, file_source.as_str()) {
        Ok(_) => {},
        Err(reason) => {
            match reason {
                ParseError::InvalidToken { location } => {
                    let (line, column) = get_line_and_column(location);
                    // Do we want the nth character or nth byte?
                    // https://stackoverflow.com/questions/30811107/getting-a-single-character-out-of-a-string
                    let character: String = file_source[location..location + 1].to_owned();
                    panic!("invalid token {} at {}:{}", character, line, column);
                },
                ParseError::UnrecognizedToken { token, expected } => {
                    match token {
                        Some(t) => {
                            let (line, column) = get_line_and_column(t.0);
                            panic!("unexpected token {:?} at {}:{}; expected one of {:?}", (t.1).1, line, column, expected);
                        },
                        None => {
                            panic!("unexpected EOF; expected one of {:?}", expected);
                        }
                    }
                },
                _ => { panic!("{:?}", reason); }
            };
        },
    };
    SceneFile {
        camera: builder.build_camera(),
        animation: builder.build_animation(),
        parameters: builder.build_render_parameters(),
        objects: builder.objects,
        lights: builder.lights,
    }
}
