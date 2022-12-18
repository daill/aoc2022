use std::fs::File;
use std::{io, usize};
use std::io::BufRead;
use std::ops::{Deref, Range};
use std::slice::RSplit;
use std::str::Chars;
use trees::{tr, fr, tree, Tree};

fn read_from_file() -> Vec<Vec<char>> {
    let mut file = File::open("input");
    let mut content: Vec<<char>> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    let tr = Tree::new(());
                    line.chars().for_each(|e| {
                        match e {
                            '[' => {},
                            ',' => {},
                            ']' => {},
                            _ => {}
                        }
                    })
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



    println!("{}", res.first().unwrap());
}


fn main() {
    let content = read_from_file();
    println!("content {:?}", &content);


    first_part(&content);



}
