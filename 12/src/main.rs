use std::fs::File;
use std::{io, usize};
use std::io::BufRead;
use std::ops::{Deref, Range};
use std::slice::RSplit;
use std::str::Chars;



fn read_from_file() -> (u32, Vec<char>) {
    let mut file = File::open("input");
    let mut content: Vec<char> = Vec::new();
    let mut length = 0;
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {

                if let Ok(line) = line {
                    if length == 0 {
                        length = line.len() as u32;
                    }

                    content.append(&mut line.chars().collect::<Vec<char>>());
                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return (length, content)
}

fn get_lin_pos(length: &u32, position: &(u32, u32)) -> usize {
    (length * position.1 + position.0) as usize
}

fn check_char_at(content: &Vec<char>, current_char: &char, length: &u32, position: &(u32, u32)) -> bool {

    let mut char_at : &char = &char::default();
    char_at = content.get(get_lin_pos(length, position)).unwrap();

    if current_char == &'S' || (char_at.clone() as i32) - current_char.clone() as i32 == 1 {
        return true;
    }
    false
}

fn walk_path(content: &Vec<char>, visited: &Vec<(u32, u32)>, length: &u32, position: &(u32, u32), paths: &mut Vec<Vec<(u32, u32)>>) {
    let mut local_visited = visited.clone();
    local_visited.push(position.clone());
    let current_char = content.get(get_lin_pos(length, position)).unwrap();
    if current_char == &'E' {
        paths.push(local_visited.clone());
    }

    if position.0 > 0 {

        let p_l = &(position.0-1, position.1);
        if !local_visited.contains(p_l) && check_char_at(content, current_char, length, p_l) {
            walk_path(content, &local_visited, length, p_l, paths);
        }
    }
    if position.0 < *length {
        let p_r = &(position.0+1, position.1);
        if !local_visited.contains(p_r) && check_char_at(content, current_char, length, p_r) {
           walk_path(content, &local_visited, length, p_r, paths);
        }
    }
    if position.1 > 0 {
        let p_d = &(position.0, position.1-1);
        if !local_visited.contains(p_d) && check_char_at(content, current_char, length, p_d) {
            walk_path(content, &local_visited, length, p_d, paths);
        }
    }
    if position.1 < (content.len() as u32)/length {
        let p_u = &(position.0, position.1+1);
        if !local_visited.contains(p_u) && check_char_at(content, current_char, length, p_u) {
            walk_path(content, &local_visited, length, p_u, paths);
        }

    }

    println!("cc {} {:?}", current_char, local_visited);
}

fn first_part(content: &Vec<char>, length: &u32)  {
    let mut visited: Vec<(u32, u32)> = Vec::new();
    let mut paths: Vec<Vec<(u32, u32)>> = Vec::new();
    walk_path(content, &mut visited, length, &(0,0), &mut paths);
}


fn main() {
    let (length, content) = read_from_file();
    println!("content {:?}", &content);


    first_part(&content, &length);



}
