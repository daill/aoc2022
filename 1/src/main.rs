extern crate core;

use core::panicking::panic;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() {
    let mut file = File::open("input");
    let content  = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<String> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    result.push(line);
                }
            }
        },
        Err(e) => panic!(e)
    };
    return content
}



fn main() {
    let content: () = read_from_file();
    println!("Hello, world!");
}
