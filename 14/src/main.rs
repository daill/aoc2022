use std::fs::File;
use std::{io, usize};
use std::any::{Any, TypeId};
use core::fmt::Debug;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::hash::Hash;
use std::io::BufRead;
use std::ops::{Deref, Range};
use std::rc::Rc;
use std::slice::RSplit;
use std::str::Chars;
use std::sync::mpsc::channel;


fn read_from_file() -> HashSet<(u32, u32)> {
    let mut file = File::open("input");
    let mut rocks: HashSet<(u32, u32)> = HashSet::new();
    match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                if let Ok(line) = line {
                    let mut path = line.
                        split(" -> ").
                        map(|point| point.
                            split_once(",").
                            map(|pair| (pair.0.parse::<u32>().unwrap(), pair.1.parse::<u32>().unwrap())).unwrap()).
                        collect::<Vec<(u32, u32)>>();
                    //path.reverse();
                    loop {
                        if !path.is_empty() {
                            let p = path.pop().unwrap();
                            println!("{:?}", p);
                        } else {
                            break;
                        }

                    }

                }
            }
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return rocks
}


fn first_part(content: &HashSet<(u32, u32)>, start_position: (u32, u32)) {
    for 
}

fn main() {
    let content: HashSet<(u32, u32)> = read_from_file();
    first_part(&content);

}
