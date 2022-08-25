use std::fs::{self, File};
extern crate colorful;

use colorful::{Color, Colorful, HSL, RGB};

pub struct Cli {
    pub args: Vec<String>
}

impl Cli {
    pub fn start(&mut self) {
        if self.args.len() < 2 {
            println!("{}", "please provide the filename!".gradient(Color::Red))
        }

        if self.args[1] == "--help" {
            println!("This is help page")
        }

        if self.args[1] == "-r" || self.args[1] == "--remove" {
            self.remove()
        }

        self.create();
    }

    fn create(&mut self) {
        for i in 1..self.args.len() {
            let result = File::create(&self.args[i]).expect("Error creating file!");

            match result {
                _file => println!("{}","�� File created successfully.".bg_color(Color::Green).color(Color::White)),
            }
        }
    }

    fn remove(&mut self) {
        for i in 1..self.args.len() {
            let result = fs::remove_file(&self.args[i]).expect("Error deleting file!");

            match result {
                file => println!("File removed successfully: {:?}", file),
            }
        }
    }
}