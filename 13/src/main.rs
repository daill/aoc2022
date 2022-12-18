use std::fs::File;
use std::{io, usize};
use std::any::Any;
use std::fmt::Debug;
use std::io::BufRead;
use std::ops::{Deref, Range};
use std::slice::RSplit;
use std::str::Chars;


trait Node {
    fn get_value(&self) -> Any;
}


impl Debug for dyn Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self of
        write!(f, "Node {}", self.get_value());
    }
}



#[derive(Debug)]
struct List {
    elements: Vec<u32>,
}

impl List {
    fn new() -> List {
        List {
            elements: Vec::new()
        }
    }
}

#[derive(Debug)]
struct Element {
    value: u32,
}

impl Element {
    fn new(val: u32) -> Element {
        Element {
            value: val,
        }
    }
}

impl Node for List {
    fn get_value(&self) -> Vec<u32> {
        (*self.elements).to_owned()
    }
}

impl Node for Element {
    fn get_value(&self) -> u32 {
        (*self.value).to_owned()
    }
}


fn read_from_file() -> Vec<Vec<Node>> {
    let mut file = File::open("input");
    let mut content: Vec<Vec<Node>> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    let mut stack_a: Vec<Node> = Vec::new();
                    let mut element = String::default();

                    line.chars().for_each(|e| {
                        match e {
                            '[' => {stack_a.push(List::new())},
                            ',' => {stack_a.last_mut().unwrap().push(Element::new(element.parse::<u32>().unwrap())); element = String::default()},
                            ']' => {},
                            _ => {element.push(e)}
                        }
                    });
                    content.push(stack_a.clone());
                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content
}

fn read_token(chars: &Vec<char>, start: usize) {

}

fn first_part(content: &Vec<Vec<char>>)  {


}


fn main() {
    let content = read_from_file();
    println!("content {:?}", &content);


    first_part(&content);



}
