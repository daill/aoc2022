use std::fs::File;
use std::{io, usize};
use std::any::{Any, TypeId};
use core::fmt::Debug;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::io::BufRead;
use std::ops::{Deref, Range};
use std::rc::Rc;
use std::slice::RSplit;
use std::str::Chars;
use std::sync::mpsc::channel;

#[derive(Debug, Clone)]
enum Token {
    Number(u32),
    List(Box<Vec<Token>>)
}



fn read_from_file() -> Vec<Token> {
    let mut file = File::open("input");
    let mut content: Vec<Token> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                let mut stack = vec![vec![]];
                let mut element = String::default();
                let idx = stack.len()-1;
                if let Ok(line) = line {
                    if line.is_empty() {
                        continue;
                    }
                    println!("line {}", line);
                    let mut element = String::default();

                    line.chars().enumerate().for_each(|(i, e)| {
                        match e {
                            '[' => {
                                stack.push(vec![]);
                            },
                            ',' => {
                                if element != String::default() {
                                    stack.last_mut().unwrap().push(Token::Number(element.parse::<u32>().unwrap()));
                                    element = String::default();
                                }
                            },
                            ']' => {
                                if element != String::default() {
                                    stack.last_mut().unwrap().push(Token::Number(element.parse::<u32>().unwrap()));
                                    element = String::default();
                                }
                                let v = stack.pop().unwrap();
                                stack.last_mut().unwrap().push(Token::List(Box::from(v)));
                            }
                            _ => {element.push(e)}
                        }
                    });
                }
                println!("content {:?} \n", stack);
                content.push(Token::List(Box::from((stack.pop().unwrap()))));
            }
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content
}



fn cmp(left: &Token, right: &Token) -> Ordering {
    match (left, right) {
        (Token::Number(left_value), Token::Number(right_value)) => {
            left_value.cmp(&right_value)
        },
        (Token::Number(left_value), Token::List(right_value)) => {
            cmp(&Token::List(Box::from(vec![Token::Number(left_value.clone())])), right)

        },
        (Token::List(left_value), Token::Number(right_value)) => {
            cmp(left, &Token::List(Box::from(vec![Token::Number(right_value.clone())])))
        }
        (Token::List(left_value), Token::List(right_value)) => {
            let mut l_iter = left_value.iter();
            let mut r_iter = right_value.iter();
            loop {
                let (left, right) = (l_iter.next(), r_iter.next());
                if left.is_none() && right.is_none() {
                    return Ordering::Equal;
                } else if left.is_some() && right.is_none() {
                    return Ordering::Greater;
                } else if left.is_none() && right.is_some() {
                    return Ordering::Less;
                } else {
                    let ret = cmp(left.unwrap(), right.unwrap());
                    if ret != Ordering::Equal {
                        return ret;
                    }
                }
            }
        }
    }
}


fn main() {
    let content: Vec<Token> = read_from_file();

    let mut cnt = 0;
    let mut id = 0;
    for pair in content.chunks(2) {
        id += 1;
        let res = cmp(&pair[0], &pair[1]);
        println!("{:?} {:?} {:?} \n", res, &pair[0], &pair[1]);
        if res == Ordering::Less {
            cnt += id;

        }
    }
    println!("{}", cnt);

    let mut two_id = 1;

    let mut six_id = 2;
    let two = Token::List(Box::from(vec![Token::List(Box::from(vec![Token::Number(2)]))]));
    let six = Token::List(Box::from(vec![Token::List(Box::from(vec![Token::Number(6)]))]));
    for pair in content.chunks(2) {
        for i in 0..2 {
            if cmp(&two, &pair[i]) == Ordering::Greater {
                two_id += 1;
            }

            if cmp( &six, &pair[i]) == Ordering::Greater {
                println!("{:?}", &pair[i]);
                six_id += 1;
            }
        }
    }

    println!("{} {}", two_id, six_id);





}
