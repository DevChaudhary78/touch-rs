use std::env;

mod lib;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    lib::Cli::start(&mut args);
}