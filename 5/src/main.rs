use std::any::Any;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::Chars;
use regex::Regex;


// a-z 1-26
// A-Z 27-52

fn read_from_file() -> (Vec<Vec<char>>, Vec<Vec<i32>>) {
    let mut file = File::open("input");
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut procedure: Vec<Vec<i32>> = Vec::new();
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

                        let mut counter = line.len()-2;
                        let mut line_counter = 0;
                        while counter < line.len() {
                            if stacks.len() == line_counter {
                                stacks.push(Vec::new());
                            }
                            let c = line.chars().nth(counter).unwrap();
                            if c.is_alphabetic() {
                                stacks[line_counter].push(c);
                            }

                            if counter < 4 {
                                break;
                            }
                            counter -= 4;

                            line_counter += 1;
                        }

                    } else {
                        for capture in re.captures_iter(&line) {
                            let a = i32::from(*&capture[1].parse::<i32>().unwrap());
                            let b = i32::from(*&capture[2].parse::<i32>().unwrap())-1;
                            let c = i32::from(*&capture[3].parse::<i32>().unwrap())-1;
                            procedure.push(Vec::from([a,b,c]));
                        }
                    }
                }
            }
            stacks.reverse();
            println!("stacks {:?}", &stacks);
            print!("{:?}", procedure);
            (stacks, procedure)
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return (stacks, procedure);
}


fn first_part(stacks: &mut Vec<Vec<char>>, procedure: &Vec<Vec<i32>>) {
    for proc in procedure {
        let amount = &proc[0];
        let from = &proc[1]-1;
        let to = &proc[2]-1;


        for a in 0..*amount {
            let b: Vec<char> = stacks.get_mut(from).unwrap();
        }

    }
}


fn main() {
    let (stacks, procedure) = read_from_file();


    //first_part(&content);




}
