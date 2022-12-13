use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, RefCell, RefMut};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::BufRead;
use std::ops::{Deref, Range};
use std::slice::RSplit;
use std::str::Chars;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i32>,
    operation: Vec<String>,
    test: Vec<String>,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items:  Vec::new(),
            operation: Vec::new(),
            test: Vec::new(),
        }
    }

    fn throw_to(&mut self, item: i32) {
        &self.items.push(item);
    }

}

fn read_from_file() -> HashMap<i32, RefCell<Monkey>> {
    let mut file = File::open("input");
    let mut content: HashMap<i32, RefCell<Monkey>> = HashMap::new();
    let mut monkey: Monkey = Monkey::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut num = 0;
            for line in lines {

                if let Ok(line) = line {
                    let trimmed = line.trim();
                    if trimmed.starts_with("Monkey") {
                        num = trimmed.split(" ").nth(1).unwrap().replace(":", "").parse::<i32>().unwrap();
                    } else if trimmed.starts_with("Starting items") {
                        trimmed.split_at(16).1.split(',').for_each(|e| monkey.items.push(e.trim().parse::<i32>().unwrap()))
                    } else if trimmed.starts_with("Operation") {
                        let ops = trimmed.split_at(12).1.split("=").nth(1).unwrap().trim().split(" ");
                        ops.for_each(|e| monkey.operation.push(String::from(e)));
                    } else if trimmed.starts_with("Test") {
                        trimmed.split_at(6).1.split(" by ").for_each(|e| monkey.test.push(String::from(e)));
                    } else if trimmed.starts_with("If true") {
                        monkey.test.push(String::from(trimmed.split_at(12).1.split("monkey").nth(1).unwrap().trim()));
                    } else if trimmed.starts_with("If false") {
                        monkey.test.push(String::from(trimmed.split_at(13).1.split("monkey").nth(1).unwrap().trim()));
                        content.insert(num, RefCell::new(monkey.clone()));
                        monkey = Monkey::new();
                    }

                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content;
}

fn get_sec(op1: &str, old: &i32) -> i32 {
    let mut sec : i32 = 0;
    if op1 == "old" {
        sec = *old;
    } else {
        sec = op1.parse::<i32>().unwrap();
    }
    return sec;
}

fn first_part(content: &mut HashMap<i32, RefCell<Monkey>>)  {
    let mut sec = 0;
    let mut keys = content.keys().into_iter().map(|e| e.clone()).collect::<Vec<i32>>();
    keys.sort();
    for i in 0..20 {
        for monkey_id in keys.iter() {

            let mut monkey = content.get(monkey_id).unwrap().borrow_mut();
            println!("{:?}", monkey);
            for item in &monkey.items {
                let mut res = match monkey.operation[1].as_str() {
                    "*" => {
                        i32::from(item * get_sec(&monkey.operation[2], &item))
                    },
                    "+" => {
                        i32::from(item + get_sec(&monkey.operation[2], &item))
                    }
                    _ => 0
                };
                res = res/3;
                let div_by = &monkey.test[1].parse::<i32>().unwrap();
                if (res / div_by) == 0 {
                    let id = &monkey.test[2].parse::<i32>().unwrap();
                    let mut to = content.get(id).unwrap().borrow_mut();
                    println!("{:?}", to);

                    to.items.push(res);
                    println!("{:?}", to);
                } else {
                    let id = &monkey.test[3].parse::<i32>().unwrap();
                     = content.get(id).unwrap().borrow_mut();
                    println!("{:?}", to);

                    to.items.push(res);
                    println!("{:?}", to);
                }
            }
            &monkey.items.clear();
        }
        content.keys().for_each(|e| println!("{:?}", content.get(e).unwrap()));
    }
}


fn main() {
    let mut content = read_from_file();
    println!("content {:?}", &content);

    first_part(&mut content);



}
