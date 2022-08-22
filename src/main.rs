use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let mut file = File::create(file_name).expect("Error encountered while creating a file");

    print!("{}", file_name);
}
