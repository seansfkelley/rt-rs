use std::path::Path;
use std::fs::File;
use std::io::Read;
use regex::Regex;

lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"//[^\n]*(\n|$)").unwrap();
}

pub fn strip_comments(s: String) -> String {
    COMMENT_REGEX.replace_all(s.as_str(), "$1").into_owned()
}

pub fn read_file_contents(path: &Path) -> String {
    let formatted_path = path.to_str().unwrap_or("input file");
    let mut contents: String = String::new();
    File::open(path)
        .expect(&format!("couldn't open {}", formatted_path))
        .read_to_string(&mut contents)
        .expect(&format!("couldn't read {} after opening", formatted_path));

    contents
}
