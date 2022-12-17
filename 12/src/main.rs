use std::fs::File;
use std::{io, usize};
use std::collections::HashMap;
use std::ffi::c_char;
use std::io::BufRead;
use std::ops::{Deref, Range};
use std::slice::RSplit;
use std::str::Chars;
use std::task::ready;

fn read_from_file() -> (u32, Vec<char>, Vec<(u32, (u32, u32))>) {
    let mut file = File::open("input");
    let mut content: Vec<char> = Vec::new();
    let mut dists: Vec<(u32, (u32, u32))> = Vec::new();
    let mut length = 0;
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for (id, line) in lines.enumerate() {

                if let Ok(line) = line {
                    if length == 0 {
                        length = line.len() as u32;
                    }
                    content.append(&mut line.chars().enumerate().map(|(i, c)| {
                        if c != 'S' {
                            dists.push((u32::MAX, (i.clone() as u32, id.clone() as u32)));
                        }
                        c
                    }).collect::<Vec<char>>());
                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return (length, content, dists)
}

fn get_lin_pos(length: &u32, position: &(u32, u32)) -> usize {
    (length * position.1 + position.0) as usize
}

fn check_char_at(content: &Vec<char>, current_char: &char, length: &u32, cur_pos: &(u32, u32), position: &(u32, u32), dists: &mut Vec<(u32, (u32, u32))>) {

    let mut char_at : &char = &char::default();
    char_at = content.get(get_lin_pos(length, position)).unwrap();

    let mut val = ((*current_char as u32)+1);
    let mut dist = 0;
    if current_char == &'S' {
        val = ('a' as u32) + 1;
        dist = 1;
    } else {
        if let Some(cur) =  dists.iter().position(|e| e.1 == *cur_pos) {
            dist = (dists.get(cur).unwrap().0) + 1;
            dists.remove(cur);
        }
    }
    if val >= (*char_at as u32) {
        if let Some(id) = dists.iter().position(|e| e.1 == *position) {
            dists.get_mut(id).unwrap().0 = u32::min(dists.get(id).unwrap().0, dist);
        }
    }
    println!("{:?}", dists);

}

fn walk_path(content: &Vec<char>, visited: &Vec<(u32, u32)>, length: &u32, position: (u32, u32), paths: &mut HashMap<u32, Vec<(u32, u32)>>, dists: &mut Vec<(u32, (u32, u32))>) {
    let mut current_char = &char::default();
    let mut cur_pos = position;
    let mut vis: Vec<(u32, u32)> = Vec::new();
    let mut ds: Vec<(i32, (u32, u32))> = Vec::new();
    loop {
        current_char = content.get(get_lin_pos(length, &cur_pos)).unwrap();
        //println!("cc {} pos {:?}", current_char, position);
        if current_char == &'E' {
            //println!("{} {:?}", local_visited.len(), local_visited);
            return;
        }


        if cur_pos.0 < *length - 1 {
            let p_r = (cur_pos.0 + 1, cur_pos.1);
            if !vis.contains(&p_r) {
               check_char_at(content, current_char, length, &cur_pos, &p_r,  dists);
            }
        }
        if cur_pos.1 < (content.len() as u32) / length - 1 {
            let p_u = (cur_pos.0, cur_pos.1 + 1);
            if !vis.contains(&p_u) {
                check_char_at(content, current_char, length,&cur_pos,  &p_u, dists);
            }
        }
        if cur_pos.1 > 0 {
            let p_d = (cur_pos.0, cur_pos.1 - 1);
            if !vis.contains(&p_d) {
                check_char_at(content, current_char, length, &cur_pos, &p_d, dists);
            }
        }
        if cur_pos.0 > 0 {
            let p_l = (cur_pos.0 - 1, cur_pos.1);

            if !vis.contains(&p_l) {
                check_char_at(content, current_char, length, &cur_pos, &p_l, dists);
            }

        }

        vis.push(cur_pos);
        cur_pos = dists.iter().min_by(|a, b| a.0.cmp(&b.0) ).map(|e| e.1 ).unwrap();
    }

}

fn first_part(content: &Vec<char>, length: &u32, mut dists: &mut Vec<(u32, (u32, u32))>)  {
    let mut visited = Vec::new();
    let mut paths: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    walk_path(content, &mut visited, length, (0,0), &mut paths, &mut dists);
    paths.into_iter().for_each(|e| println!("{} {:?}", e.0, e.1));
}


fn main() {
    let (length, content, mut dists) = read_from_file();
    println!("content {:?}", &content);


    first_part(&content, &length, &mut dists);



}
