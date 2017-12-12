use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod lib;

fn main() {
    let s = get_file_string();
    let steps = lib::solve_first(&*s);
    println!("shortest amount of steps: \n{}", steps);

    let steps = lib::solve_second(&*s);
    println!("Max amount of steps: \n{}", steps);

}

fn get_file_string() -> String {
    // Create a path to the desired file
    let path = Path::new("directions.txt");
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
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => print!("{} read\n", display),
    }
    return s;
}