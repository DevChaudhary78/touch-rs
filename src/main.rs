use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        let file_name = &args[i];

        File::create(file_name).expect("Error encountered while creating a file.");
    }
}
