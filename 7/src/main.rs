use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::io::BufRead;
use std::str::Chars;



fn read_from_file() -> (Vec<Vec<char>>) {
    let mut file = File::open("input");
    let mut content: Vec<Vec<char>> = Vec::new();
    let mut fs = tr("/");
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                if let Ok(line) = line {
                    println!("line {}", line);
                    content.push(line.chars().collect::<Vec<char>>());

                    if line.starts_with("$ ") {
                        // command
                        if line.matches()
                    } else {
                        // file and size
                    }

                }
            }
            println!("content {:?}", &content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content;
}


fn first_part(content: &Vec<Vec<char>>) {

}


fn main() {
    let content = read_from_file();


    first_part(&content);




}
