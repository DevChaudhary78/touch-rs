use std::fs::{self, File};

use console::style;

pub struct Cli {
    pub args: Vec<String>,
}

impl Cli {
    pub fn start(&mut self) {
        if self.args.len() < 2 {
            eprintln!("Syntax: {}", style("touch filename.ext").cyan());
            return;
        }

        if self.args[1] == "-h" || self.args[1] == "--help" {
            println!("Syntax: {}", style("touch filename.ext").red());

            println!("touch <Options> filename");

            println!("<Options>");
            return;
        }

        if self.args[1] == "-r" || self.args[1] == "--remove" {
            self.remove();
            return;
        }

        self.create();
    }

    fn create(&mut self) {
        for i in 1..self.args.len() {
            File::create(&self.args[i]).expect("Error creating file!");
        }

        print!("{}: [", style("Files created succesfully:").green());
        for i in 1..self.args.len() {
            print!(" {} ", self.args[i]);
        }
        print!("]");
    }

    fn remove(&mut self) {
        for i in 2..self.args.len() {
            fs::remove_file(&self.args[i]).expect("Error removing file.");
        }

        print!("{}: [", style("File deleted succesfully:").red());

        for i in 2..self.args.len() {
            print!(" {} ", self.args[i]);
        }

        print!("]");
    }
}
