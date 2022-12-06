use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::io::BufRead;
use std::str::Chars;
use regex::Regex;


// a-z 1-26
// A-Z 27-52

fn read_from_file() -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut file = File::open("input");
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut procedure: Vec<Vec<usize>> = Vec::new();
    let (stacks, procedure) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let re = Regex::new(r"\w+\W(\d+)\W\w+\W(\d+)\W\w+\W(\d+)").unwrap();
            let mut procedure_part: bool = false;
            for line in lines {
                if let Ok(line) = line {
                    println!("line {}", line);
                    if line.is_empty() {
                        procedure_part = true;

                        continue;
                    }
                    if procedure_part == false {

                        let mut counter = 1;
                        let mut line_counter = 0;
                        while counter < line.len() {
                            if stacks.len() == line_counter {
                                stacks.push(Vec::new());
                            }
                            let c = line.chars().nth(counter).unwrap();
                            if c.is_alphabetic() {
                                stacks[line_counter].insert(0, c);
                            }


                            counter += 4;

                            line_counter += 1;
                        }

                    } else {
                        for capture in re.captures_iter(&line) {
                            let a = usize::from(*&capture[1].parse::<usize>().unwrap());
                            let b = usize::from(*&capture[2].parse::<usize>().unwrap())-1;
                            let c = usize::from(*&capture[3].parse::<usize>().unwrap())-1;
                            procedure.push(Vec::from([a,b,c]));
                        }
                    }
                }
            }
            println!("stacks {:?}", &stacks);
            print!("{:?}", procedure);
            (stacks, procedure)
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return (stacks, procedure);
}


fn first_part(stacks: &mut Vec<Vec<char>>, procedure: Vec<Vec<usize>>) {
    for proc in procedure {
        let amount = proc[0];
        let from = proc[1];
        let to = proc[2];

        println!("amount {} from {} to {}", amount, from, to);
        for a in 0..amount {
            let from_vec: &mut Vec<char> = stacks.get_mut(from).unwrap();
            let mov = from_vec.last().unwrap().clone();
            from_vec.truncate(from_vec.len()-1);
            let to_vec: &mut Vec<char> = stacks.get_mut(to).unwrap();
            to_vec.push(mov);
            println!("{:?}", to_vec);
        }

    }
    stacks.iter().for_each(|stack| print!("{}", stack.last().unwrap()));
}


fn main() {
    let (mut stacks, procedure) = read_from_file();


    first_part(&mut stacks, procedure);



}
