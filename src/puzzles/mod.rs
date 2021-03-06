pub mod puzzle1;
pub mod puzzle2;
pub mod puzzle2b;
pub mod puzzle3;
pub mod puzzle4;
pub mod puzzle5;
pub mod puzzle6;
pub mod puzzle7;
pub mod puzzle8;
pub mod puzzle9;
pub mod puzzle10    ;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


//common puzzle funcs
pub fn read_puzzle_input(file_name: &str) -> String {
    let full_file_path : String = "./puzzle-inputs/".to_string() + file_name;
    file_to_text(&full_file_path)
}

fn file_to_text(file_name : &str) -> String {

    let path = Path::new(file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let file_contents = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) =>  s
    };
    file_contents
}

//can't figure out how to easily go from Vec<char> -> String
#[allow(dead_code)]
fn to_string(chars : &[char]) -> String {
    let vec_of_chars : Vec<&char> = chars.into_iter().collect();
    vec_of_chars.iter().map(|x| **x).collect::<String>()
}