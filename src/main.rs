use std::env;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut initialization = lib::Cli {
        args: args,
    };

    lib::Cli::start(&mut initialization);
}