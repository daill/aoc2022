use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::io::BufRead;
use std::str::Chars;


// a-z 1-26
// A-Z 27-52

fn read_from_file() -> (Vec<Vec<char>>) {
    let mut file = File::open("input");
    let mut content: Vec<Vec<char>> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                if let Ok(line) = line {
                    println!("line {}", line);
                    content.push(line.chars().collect::<Vec<char>>());
                }
            }
            println!("stacks {:?}", &content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content;
}


fn first_part(content: &Vec<Vec<char>>) {
    for line in content.iter() {
        println!("{:?}", line);
        for (idx, window) in line.windows(4).enumerate() {
            let mut vec = Vec::from(window);
            vec.sort();

            vec.dedup();
            if vec.len() == 4 {
                println!("{}", idx+4);
                break;
            }
        }
    }
}


fn second_part(content: &Vec<Vec<char>>) {
    for line in content.iter() {
        println!("{:?}", line);
        for (idx, window) in line.windows(14).enumerate() {
            let mut vec = Vec::from(window);
            vec.sort();

            vec.dedup();
            if vec.len() == 14 {
                println!("{}", idx+14);
                break;
            }
        }
    }
}

fn main() {
    let content = read_from_file();


    //first_part(&content);
    second_part(&content);



}
