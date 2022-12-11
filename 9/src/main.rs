use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::slice::RSplit;
use std::str::Chars;



fn read_from_file() -> Vec<(String, usize)> {
    let mut file = File::open("input");
    let mut content: Vec<(String, usize)> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    println!("line {}", line);
                    let (a, b) = line.split_at(2);
                    content.push((String::from(a.trim()), b.parse::<usize>().unwrap()));
                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content;
}


fn second_part(content: &Vec<(String, usize)>)  {
    let mut rope : Vec<(i32, i32)> = vec![(0,0); 10];
    let mut his : HashSet<(i32, i32)> = HashSet::new();

    for mov in content {
        for i in 0..mov.1 {
            round_move(&mut rope.first_mut().unwrap(), mov.0.as_str());

            for i in 1..rope.len() {
                let (h, t) = rope.split_at_mut(i);
                let head = h.last().unwrap();
                let tail = t.first_mut().unwrap();
                if (head.1-tail.1).abs() > 1 ||  (head.0-tail.0).abs() > 1 {
                    move_to(tail, head);
                }
            }

            println!("m {} c {} {:?}", mov.0, i, rope);
            his.insert(rope[9].clone());
        }
    }
    println!("c {} {:?}", his.len(), his);
}

fn move_to(mut p2: &mut (i32, i32), p1: &(i32, i32)) {
    p2.0 += (p1.0-p2.0).signum();
    p2.1 += (p1.1-p2.1).signum();
}

fn first_part(content: &Vec<(String, usize)>)  {
    let mut h = (0,0);
    let mut t = (0,0);
    let mut h_his: Vec<(i32, i32)> = Vec::new();
    let mut t_his: HashSet<(i32, i32)> = HashSet::new();

    for mov in content {
        for i in 0..mov.1 {
            round_move(&mut h,  mov.0.as_str());
            if (h.1-t.1).abs() > 1 ||  (h.0-t.0).abs() > 1 {
                t = *h_his.last().unwrap();
                t_his.insert(t);
            }

            h_his.push(h.clone());
            println!("{:?} {:?}", h, t);
        }
    }

    println!("count {} {:?}", t_his.len(), t_his);
}

fn round_move(mut h: &mut (i32, i32), dir: &str) {
    match dir {
        "R" => {
            h.0 += 1;
        },
        "U" => {
            h.1 += 1;
        },
        "L" => {
            h.0 -= 1;
        },
        "D" => {
            h.1 -= 1;
        },
        _ => {}
    }
    // t follow
}


fn main() {
    let content = read_from_file();
    println!("content {:?}", &content);

    //first_part(&content);
    second_part(&content);




}
