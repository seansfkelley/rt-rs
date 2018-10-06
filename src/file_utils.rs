use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn read_file_contents(path: &Path) -> String {
    let formatted_path = path.to_str().unwrap_or("input file");
    let mut contents: String = String::new();
    File::open(path)
        .expect(&format!("couldn't open {}", formatted_path))
        .read_to_string(&mut contents)
        .expect(&format!("couldn't read {} after opening", formatted_path));

    contents
}
