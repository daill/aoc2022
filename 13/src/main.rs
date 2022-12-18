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

#[derive(Debug)]
enum TreeType {
    Value,
    Branch,
}

#[derive(Debug)]
struct Tree {
    tree_type: TreeType,
    parent: Option<Rc<Tree>>,
    left: Option<Rc<Tree>>,
    right: Option<Rc<Tree>>,
    values: Vec<u32>,
}

impl Tree {
    fn new_branch() -> Tree {
        Tree {
            tree_type: TreeType::Branch,
            parent: None,
            left: None,
            right: None,
            values: vec![],
        }
    }

    fn new_value() -> Tree {
        Tree {
            tree_type: TreeType::Value,
            parent: None,
            left: None,
            right: None,
            values: vec![],
        }
    }

    fn add_child(&mut self, child: Tree) {
        if self.left.is_none() {
            self.left = Option::from(Rc::new(child));
        } else {
            self.right = Option::from(Rc::new(child));
        }
    }
}

fn read_from_file() -> Vec<Tree> {
    let mut file = File::open("input");
    let mut content: Vec<Tree> = Vec::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    if line.is_empty() {
                        continue;
                    }
                    let mut root = Tree::new_branch();
                    let mut current = &mut root;
                    content.push(root);

                    let mut element = String::default();

                    line.chars().skip(1).for_each(|e| {
                        match e {
                            '[' => {
                                let mut a = Tree::new_branch();
                                current.add_child(a);
                                current = &mut a;
                            },
                            ','|']' => {

                            },
                            _ => {element.push(e)}
                        }
                    });
                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content
}

fn read_token(chars: &Vec<char>, start: usize) {

}

fn first_part(content: &Vec<Tree>)  {


}


fn main() {
    let content: Vec<Tree> = read_from_file();
    println!("content {:?}", &content);


    first_part(&content);



}
