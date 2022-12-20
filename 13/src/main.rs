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
                                    let idx = stack.len()-1;
                                    stack.get_mut(idx).unwrap().push(Token::Number(element.parse::<u32>().unwrap()));
                                    element = String::default();
                                }
                            },
                            ']' => {
                                if element != String::default() {
                                    let idx = stack.len()-1;
                                    stack.get_mut(idx).unwrap().push(Token::Number(element.parse::<u32>().unwrap()));
                                    element = String::default();
                                }
                                let v = stack.pop().unwrap();
                                let idx = stack.len()-1;
                                stack.get_mut(idx).unwrap().push(Token::List(Box::from(v)));
                            }
                            _ => {element.push(e)}
                        }
                    });
                }
                println!("content {:?}", stack);
                content.p
            }
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content
}

fn read_token(chars: &Vec<char>, start: usize) {

}

fn first_part(content: &Vec<Token>)  {


}


fn main() {
    let content: Vec<Token> = read_from_file();
    println!("content {:?}", &content);


    first_part(&content);



}
