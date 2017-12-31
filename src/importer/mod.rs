mod parser;
mod scene_builder;

use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use core::*;
use lalrpop_util::ParseError;
use self::scene_builder::SceneBuilder;

lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"//[^\n]*(\n|$)").unwrap();
    static ref NEWLINE_REGEX: Regex = Regex::new(r"\n").unwrap();
}

fn read_file_contents(path: &Path) -> String {
    let formatted_path = path.to_str().unwrap_or("input file");
    let mut contents: String = String::new();
    File::open(path)
        .expect(&format!("couldn't open {}", formatted_path))
        .read_to_string(&mut contents)
        .expect(&format!("couldn't read {} after opening", formatted_path));

    contents
}

fn strip_comments(s: String) -> String {
    // It is VERY IMPORTANT that this regex does not drop newlines, else our line counts will be off.
    COMMENT_REGEX.replace_all(s.as_str(), "$1").into_owned()
}

#[derive(Debug)]
pub struct SceneFile {
    pub camera: Camera,
    pub animation: (u32, Transform),
    pub parameters: RenderParamaters,
    pub objects: Vec<SceneObject>,
    pub lights: Vec<LightType>,
}

pub fn parse(path: &Path) -> SceneFile {
    let mut builder = SceneBuilder::new();
    parse_into_builder(path, &mut builder, parser::parse_SceneFile);
    SceneFile {
        camera: builder.build_camera(),
        animation: builder.build_animation(),
        parameters: builder.build_render_parameters(),
        objects: builder.objects,
        lights: builder.lights,
    }
}

type ParserFn<T> = for<'a> fn(&mut SceneBuilder, &Path, &'a str) -> Result<T, ParseError<usize, (usize, &'a str), ()>>;

// https://stackoverflow.com/questions/48038871/value-does-not-live-long-enough-but-only-when-using-a-function-pointer
pub fn parse_into_builder<T>(path: &Path, builder: &mut SceneBuilder, method: ParserFn<T>) -> T {
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

    match method(builder, path, file_source.as_str()) {
        Ok(value) => { value },
        Err(reason) => {
            let formatted_file_name = path.to_str().expect("could not convert path to string");
            match reason {
                ParseError::InvalidToken { location } => {
                    let (line, column) = get_line_and_column(location);
                    // Do we want the nth character or nth byte?
                    // https://stackoverflow.com/questions/30811107/getting-a-single-character-out-of-a-string
                    let character: String = file_source[location..location + 1].to_owned();
                    panic!("invalid token \"{}\" at {}:{}:{}", character, formatted_file_name, line, column);
                },
                ParseError::UnrecognizedToken { token, expected } => {
                    let formatted_expected = expected.join(", ");
                    match token {
                        Some(t) => {
                            let (line, column) = get_line_and_column(t.0);
                            panic!("unexpected token \"{}\" at {}:{}:{}; expected one of {}", (t.1).1, formatted_file_name, line, column, formatted_expected);
                        },
                        None => {
                            panic!("unexpected EOF in {}; expected one of {}", formatted_file_name, formatted_expected);
                        }
                    }
                },
                _ => { panic!("{:?}", reason); }
            };
        },
    }
}
