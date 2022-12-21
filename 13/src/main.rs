use std::fs::File;
use std::{io, usize};
use std::any::{Any, TypeId};
use core::fmt::Debug;
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
                println!("content {:?}", stack);
                content.push(Token::List(Box::from((stack.pop().unwrap()))));
            }
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content
}



fn cmp(left: &Token, right: &Token) -> bool {
    match (left, right) {
        (Token::Number(left_value), Token::Number(right_value)) => {
            println!("{} {}", left_value, right_value);
            if left_value > right_value {
                return false;
            }
            true
        },
        (Token::Number(left_value), Token::List(right_value)) => {
            cmp(&Token::List(Box::from(vec![Token::Number(left_value.clone())])), right)
        },
        (Token::List(left_value), Token::Number(right_value)) => {
            cmp(left, &Token::List(Box::from(vec![Token::Number(right_value.clone())])))
        }
        (Token::List(left_value), Token::List(right_value)) => {
            let mut i = 0;
            let mut ret = false;
            loop {
                if i < left_value.len() && i < right_value.len() {
                    if let (Some(l_token), Some(r_token)) = (left_value.get(i), right_value.get(i)) {
                        ret = cmp(l_token, r_token);
                        if ret == false {
                            break;
                        }
                    }
                } else if i >= left_value.len() && i < right_value.len(){
                    ret = true;
                    break;
                } else {
                    break;
                }
                i += 1;

            }
            ret
        }
        _ => false
    }
}


fn main() {
    let content: Vec<Token> = read_from_file();

    for i in (0..content.len()).step_by(2) {
        let left = content.get(i).unwrap();
        let right = content.get(i+1).unwrap();

        println!("{}", cmp(left, right));
    }

    println!("content {:?}", &content);





}
