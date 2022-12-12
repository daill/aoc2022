use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::slice::RSplit;
use std::str::Chars;



fn read_from_file() -> Vec<(String, usize)> {
    let mut file = File::open("input");
    let mut content: Vec<(String, usize)> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    println!("line {}", line);
                    let (a, b) = line.split_at(2);
                    content.push((String::from(a.trim()), b.parse::<usize>().unwrap_or(0)));

                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content;
}




fn first_part(content: &Vec<(String, usize)>)  {

}


fn main() {
    let content = read_from_file();
    println!("content {:?}", &content);

    first_part(&content);




}
