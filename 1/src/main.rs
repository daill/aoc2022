extern crate core;

use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> Vec<String> {
    let mut file = File::open("input");
    let result:Vec<String> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<String> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    result.push(line);
                }
            }
            result
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    result
}



fn main() {
    let content: Vec<String> = read_from_file();
    let mut elves: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    let mut cnt: i32 = 1;
    for line in content {
        if line.eq("") {
            elves.push(sum);
            sum = 0;
            cnt += 1;
        } else {
            sum += line.parse::<i32>().unwrap();
            print!("{}", line);
            println!("next");
        }
    }
    let mut high = 0;
    let max = elves.iter().max().unwrap();
    let elve = elves.iter().position(|&r| r == *max).unwrap();
    println!("elve {} {}", elve, max);
    elves.sort();
    elves.reverse();
    println!("{}", elves[0]+elves[1]+elves[2])

}
