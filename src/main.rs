use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    File::create(file_name).expect("Error encountered while creating a file");

    print!("{}", file_name);
}
