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


                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return content;
}


fn first_part(content: &mut HashMap<i32, RefCell<Monkey>>)  {

}


fn main() {
    let mut content = read_from_file();
    println!("content {:?}", &content);

    first_part(&mut content);



}
