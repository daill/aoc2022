use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::Chars;


// a-z 1-26
// A-Z 27-52

fn read_from_file() -> Vec<Vec<Vec<i32>>> {
    let mut file = File::open("input");
    let result:Vec<Vec<Vec<i32>>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<Vec<i32>>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let splitted = line.split(",").collect::<Vec<&str>>();
                    let left = splitted[0].split("-").map(|v| v.parse().unwrap()).collect::<Vec<i32>>();
                    let right = splitted[1].split("-").map(|v| v.parse().unwrap()).collect::<Vec<i32>>();
                    result.push(Vec::from([left, right]));
                }
            }
            result
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    result
}


fn first_part(content: &Vec<Vec<Vec<i32>>>) {
    let mut res = 0;
    for line in content {
        let left = &line[0];
        let right = &line[1];
        println!("{} {} {} {}", left[0], left[1], right[0], right[1]);
        if left[0] <= right[0] && left[1] >= right[0] {
            println!("lower in upper ");
            res += 1;
        } else if left[0] <= right[1] && left[1] >= right[1]  {
            println!("upper in lower ");
            res += 1;
        } else if left[0] >= right[0] && left[1] <= right[0] {
            println!("lower in upper ");
            res += 1;
        } else if left[1] >= right[0] && left[1] <= right[1]  {
            println!("upper in lower ");
            res += 1;
        }
    }
    println!("{}", res);
}


fn main() {
    let content = read_from_file();

    first_part(&content);




}
