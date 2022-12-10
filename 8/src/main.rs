use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use std::slice::RSplit;
use std::str::Chars;



fn read_from_file() -> HashMap<String, Vec<(String, String)>> {
    let mut file = File::open("input");
    let mut content: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut files: &mut Vec<(String, String)> = &mut Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    println!("line {}", line);
                }
            }

            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    println!("content {:?}", &content);
    return content;
}


fn first_part(content: &HashMap<String, Vec<(String, String)>>)  {


}


fn main() {
    let content = read_from_file();


    first_part(&content);




}
