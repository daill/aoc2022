use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::Chars;
use regex::Regex;


// a-z 1-26
// A-Z 27-52

fn read_from_file() -> Vec<Vec<char>> {
    let mut file = File::open("input");
    let result:Vec<Vec<char>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let re = Regex::new(r"\w+\W(\d+)\W\w+\W(\d+)\W\w+\W(\d+)").unwrap();
            let mut result: Vec<Vec<char>> = Vec::new();
            let mut procedure: Vec<Vec<i32>> = Vec::new();
            let mut procedure_part: bool = false;
            for line in lines {
                if let Ok(line) = line {
                    println!("{}", line);
                    if line.is_empty() {
                        procedure_part = true;
                        continue;
                    }
                    if procedure_part == false {
                        let mut line_vec: Vec<char> = Vec::new();
                        let mut counter = 1;
                        while counter < line.len() {
                            line_vec.push(line.chars().nth(counter).unwrap());
                            counter += 4;
                        }
                        println!("{:?}", line_vec);
                        result.push(line_vec);
                    } else {
                        for capture in re.captures_iter(&line) {
                            print!("{}", &capture[1].parse::<i32>().unwrap())
                        }
                        println!("{}", line);
                    }
                }
            }
            result
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    result
}


fn first_part(content: &Vec<Vec<char>>) {

}


fn main() {
    let content = read_from_file();

    first_part(&content);




}
