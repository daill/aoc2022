use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::slice::RSplit;
use std::str::Chars;



fn read_from_file() -> Vec<(String, i32)> {
    let mut file = File::open("input");
    let mut content: Vec<(String, i32)> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    let (a, b) = line.split_at(4);
                    println!("line {} a {:?} b {:?}", line, a,b.trim());
                    content.push((String::from(a), b.trim().parse::<i32>().unwrap_or(0)));

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
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for cmd in content {
        for i in 0..process_cmd(&cmd.0) {
            cycle += 1;
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    let tmp = x*cycle;
                    println!("=> {} {} {}", x, cycle, tmp);
                    sum += tmp;
                },
                _ => {}
            }
        }
        x += cmd.1;
    }
    println!("{}", sum);
}

fn process_cmd(cmd: &String) -> i32 {
    match cmd.as_str() {
        "noop" => 1,
        "addx" => 2,
        _ => 0
    }
}


fn main() {
    let content = read_from_file();
    println!("content {:?}", &content);

    first_part(&content);


}
