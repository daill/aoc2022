use std::fs::File;
use std::io;
use std::io::BufRead;

// A/X > C/Z
// C/Z > B/Y
// B/Y > A/X
// A/X -> 1
// B/Y -> 2
// C/Z -> 3
// loss + 0
// win + 6
// draw + 3


fn read_from_file() -> Vec<Vec<String>> {
    let mut file = File::open("input");
    let result:Vec<Vec<String>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<String>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    result.push(line.split(" ").map(str::to_string).collect());
                }
            }
            result
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    result
}

fn first_part(content: &Vec<Vec<String>>) {
    let mut score: i32 = 0;
    for strat in content {
        println!("{} {}", strat[0], score);
        score += match strat[1].as_str() {
            "X" => {
                if strat[0] == "C" {
                    (6+1)
                } else if strat[0] == "A" {
                    (3+1)
                } else {
                    1
                }
            },
            "Y" => {
                if strat[0] == "A" {
                    (6+2)
                } else if strat[0] == "B" {
                    (3+2)
                } else {
                    2
                }
            },
            "Z" => {
                if strat[0] == "B" {
                    (6+3)
                } else if strat[0] == "C" {
                    (3+3)
                } else {
                    3
                }
            },
            _ => { 0 }
        }
    }
    println!("score {}", score);
}

fn second_part(content: &Vec<Vec<String>>) {
    let mut score: i32 = 0;
    for strat in content {
        score += match strat[1].as_str() {
            "X" => {
                if strat[0] == "A" {
                    (0+3)
                } else if strat[0] == "B" {
                    (0+1)
                } else if strat[0] == "C" {
                    (0+2)
                } else {
                    0
                }
            },
            "Y" => {
                if strat[0] == "A" {
                    (3+1)
                } else if strat[0] == "B" {
                    (3+2)
                } else if strat[0] == "C" {
                    (3+3)
                } else {
                    0
                }
            },
            "Z" => {
                if strat[0] == "A" {
                    (6+2)
                } else if strat[0] == "B" {
                    (6+3)
                } else if strat[0] == "C" {
                    (6+1)
                } else {
                    0
                }
            },
            _ => { 0 }
        };
        println!("{} {}", strat[0], score);
    }
    println!("score {}", score);
}

fn main() {
    let content = read_from_file();
    //first_part(&content);
    second_part(&content);

}
