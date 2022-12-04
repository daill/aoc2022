use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::Chars;


// a-z 1-26
// A-Z 27-52

fn read_from_file() -> Vec<Vec<String>> {
    let mut file = File::open("input");
    let result:Vec<Vec<String>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<String>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let splitted = line.split(",");
                    let left = String::from(splitted[0]);
                    let right = String::from(splitted[1]);
                    result.push(Vec::from([a, b]));
                }
            }
            result
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    result
}


fn first_part(content: &Vec<Vec<String>>) {

}


fn main() {
    let content = read_from_file();

    first_part(&content, &alphabet);




}
