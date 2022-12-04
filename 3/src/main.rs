use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::Chars;


// a-z 1-26
// A-Z 27-52

fn read_from_file() -> Vec<Vec<Vec<char>>> {
    let mut file = File::open("input");
    let result:Vec<Vec<Vec<char>>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<Vec<char>>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let a = String::from(&line[0..line.len()/2]).chars().collect::<Vec<char>>();
                    let b = String::from(&line[line.len()/2..line.len()]).chars().collect::<Vec<char>>();
                    result.push(Vec::from([a, b]));
                }
            }
            result
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    result
}


fn read_from_file_one_line() -> Vec<Vec<char>> {
    let mut file = File::open("input");
    let result:Vec<Vec<char>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<char>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let a = String::from(line).chars().collect::<Vec<char>>();
                    result.push(a);
                }
            }
            result
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    result
}

fn first_part(content: &Vec<Vec<Vec<char>>>, alphabet: &Vec<char>) {

    let mut res = 0;
    'outer: for bagpack in content {
        for i in 0..bagpack[0].len() {
            let ch = bagpack[0][i];
            for j in 0..bagpack[1].len() {
                if bagpack[1][j] == ch {
                    println!("{} {} {}", bagpack[0].iter().collect::<String>(), bagpack[1].iter().collect::<String>(), ch);
                    res += alphabet.iter().position(|c| c == &ch).unwrap()+1;
                    continue 'outer;
                }
            }
        }
    }

    println!("{}", res);
}

fn second_part(content: &Vec<Vec<char>>, alphabet: &Vec<char>) {
    let mut res = 0;
    'outer: for index in (0..content.len()).step_by(3) {
        println!("{}", index);
        let a = &content[index];
        let b = &content[index+1];
        let c = &content[index+2];

        for i in 0..a.len() {
            let ch = &a[i];

            for j in 0..b.len() {
                if b[j] == *ch {
                    for k in 0..c.len() {
                        if c[k] == *ch {
                            println!("{} {}", ch, alphabet.iter().position(|c| c == ch).unwrap());
                            res += alphabet.iter().position(|c| c == ch).unwrap()+1;
                            continue 'outer;
                        }
                    }
                }

            }
        }


    }
    println!("{}", res);
}

fn main() {
    let content = read_from_file();
    let mut alphabet = ('a'..='z').filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
    alphabet.extend( ('A'..='Z').filter(|c| c.is_alphabetic()).collect::<Vec<_>>());

    //first_part(&content, &alphabet);
    second_part(&read_from_file_one_line(), &alphabet);



}
