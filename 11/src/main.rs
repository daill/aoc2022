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
use gcd::Gcd;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i128>,
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

    fn throw_to(&mut self, item: i128) {
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
                        trimmed.split_at(16).1.split(',').for_each(|e| monkey.items.push(e.trim().parse::<i128>().unwrap()))
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

fn get_sec(op1: &str, old: &i128) -> i128 {
    let mut sec : i128 = 0;
    if op1 == "old" {
        sec = *old;
    } else {
        sec = op1.parse::<i128>().unwrap();
    }
    return sec;
}

fn first_part(content: &mut HashMap<i32, RefCell<Monkey>>)  {
    let mut keys = content.keys().into_iter().map(|e| e.clone()).collect::<Vec<i32>>();
    keys.sort();
    let divs = content.values().into_iter().map(|m| m.borrow().test[1].parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut lcm = divs.first().unwrap().clone();
    for d in 0..divs.len() {
        let v = divs.get(d).unwrap();
        lcm = lcm * v / lcm.gcd(*v);
    }
    println!("{}", lcm);

    let mut m_count:HashMap<i32, i128> = HashMap::new();
    for i in 0..10000 {
        for monkey_id in keys.iter() {

            let mut monkey = content.get(monkey_id).unwrap().borrow_mut();
            //println!("id {} monkey {:?}", monkey_id, monkey);
            for item in &monkey.items {
                let mut res = match monkey.operation[1].as_str() {
                    "*" => {
                        i128::from( item * get_sec(&monkey.operation[2], &item) )
                    },
                    "+" => {
                        i128::from(item + get_sec(&monkey.operation[2], &item))
                    }
                    _ => 0
                };
                //res = res/3;
                let div_by = &monkey.test[1].parse::<i128>().unwrap();
                res = res % (lcm as i128);
                if (res % div_by) == 0 {
                    let id = &monkey.test[2].parse::<i32>().unwrap();
                    let mut to = content.get(id).unwrap().borrow_mut();

                    to.items.push(res);
                    //println!("res {} to {} {:?}", res, id, to);
                } else {
                    let id = &monkey.test[3].parse::<i32>().unwrap();
                    let mut to= content.get(id).unwrap().borrow_mut();

                    to.items.push(res);
                    //println!("res {} to {} {:?}", res, id, to);
                }
            }
            let c = m_count.get(monkey_id).unwrap_or(&0) + monkey.items.len() as i128 ;
            m_count.insert(monkey_id.clone(), i128::from(c));
            &monkey.items.clear();
        }
        //content.keys().for_each(|e| println!("{:?}", content.get(e).unwrap()));
    }

    println!("{:?}", m_count);
    let mut v = m_count.into_iter().map(|e| e.1).collect::<Vec<i128>>();
    v.sort();
    v.reverse();

    println!("{}", v[0]*v[1]);
}


fn main() {
    let mut content = read_from_file();
    println!("content {:?}", &content);

    first_part(&mut content);



}
