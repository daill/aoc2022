use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::Range;
use std::slice::RSplit;
use std::str::Chars;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: fn(i32, i32) -> i32,
    test: fn(i32, i32),
}

fn read_from_file() -> Vec<(String, i32)> {
    let mut file = File::open("input");
    let mut content: Vec<(String, i32)> = Vec::new();
    let mut monkey: Monkey;
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    let trimmed = line.trim();
                    if trimmed.starts_with("Monkey") {
                        monkey = Monkey();

                        trimmed.split(" ").nth(2).unwrap();
                    } else if trimmed.starts_with("Starting items") {

                    } else if trimmed.starts_with("Operation") {

                    } else if trimmed.starts_with("Test") {

                    } else if trimmed.starts_with("If true") {

                    } else if trimmed.starts_with("If false") {

                    }

                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content;
}




fn first_part(content: &Vec<(String, i32)>)  {

}


fn main() {
    let content = read_from_file();
    println!("content {:?}", &content);

    first_part(&content);



}
