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


fn read_from_file() -> HashSet<(i32, i32)> {
    let mut file = File::open("input");
    let mut paths : HashSet<(i32, i32)> = HashSet::new();
    match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                if let Ok(line) = line {
                    let mut rocks = &mut line.
                    split(" -> ").
                        map(|point| point.
                            split_once(",").
                            map(|pair| (pair.0.parse::<i32>().unwrap(), pair.1.parse::<i32>().unwrap())).unwrap()).
                        collect::<Vec<(i32, i32)>>();
                    rocks.reverse();
                    loop {
                        if let Some(rock) = rocks.pop() {
                            if let Some(next_rock) = rocks.last() {
                                if rock.0 == next_rock.0 {
                                    let mut ys = (rock.1..next_rock.1+1);
                                    if rock.1 > next_rock.1{
                                        ys = (next_rock.1..rock.1+1);
                                    }
                                    paths.extend(ys.map(|y| (rock.0, y)).collect::<HashSet<(i32,i32)>>());
                                } else {
                                    let mut xs = (rock.0..next_rock.0+1);
                                    if rock.0 > next_rock.0{
                                        xs = (next_rock.0..rock.0+1);
                                    }
                                    paths.extend(xs.map(|x| (x, rock.1)).collect::<HashSet<(i32,i32)>>());
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };

    return paths;
}


fn second_part(content: &HashSet<(i32, i32)>, start_position: (i32, i32)) {
    let mut snowflakes: HashSet<(i32, i32)> = HashSet::new();

    let high = content.iter().filter(|e| e.0 == start_position.0).min_by(|e, f| e.1.cmp(&f.1)).unwrap().1;
    let low = content.iter().max_by(|e, f| e.1.cmp(&f.1)).unwrap().1+2;

    // start flake
    let mut current_y = high-1;
    let mut it = (start_position.0, current_y);
    snowflakes.insert(it);
    let mut cnt = 0;

    'outer: loop {
        let mut next_flake = (start_position.0, current_y);
        cnt += 1;

        loop {

            if !content.contains(&(next_flake.0, next_flake.1+1)) && !snowflakes.contains(&(next_flake.0, next_flake.1+1)) && !(next_flake.1+1 == low as i32) {
                next_flake = (next_flake.0, next_flake.1+1);
            } else if !content.contains(&(next_flake.0-1, next_flake.1+1)) && !snowflakes.contains(&(next_flake.0-1, next_flake.1+1)) && !(next_flake.1+1 == low as i32){
                next_flake = (next_flake.0-1, next_flake.1+1);
            } else if !content.contains(&(next_flake.0+1, next_flake.1+1)) && !snowflakes.contains(&(next_flake.0+1, next_flake.1+1)) && !(next_flake.1+1 == low as i32){
                next_flake = (next_flake.0+1, next_flake.1+1);
            } else {
                break;
            }

        }
        if next_flake.1 == current_y {
            current_y -= 1;
        }
        println!("added {:?}", next_flake);
        snowflakes.insert(next_flake);

        if next_flake == start_position {
            break 'outer;
        }
    }
    println!("{} {}", cnt, snowflakes.len());
}

fn first_part(content: &HashSet<(i32, i32)>, start_position: (i32, i32)) {
    let mut snowflakes: HashSet<(i32, i32)> = HashSet::new();

    let high = content.iter().filter(|e| e.0 == start_position.0).min_by(|e, f| e.1.cmp(&f.1)).unwrap().1;
    let low = content.iter().max_by(|e, f| e.1.cmp(&f.1)).unwrap().1;

    // start flake
    let mut current_y = high-1;
    let mut it = (start_position.0, current_y);
    snowflakes.insert(it);
    let mut cnt = 0;

    'outer: loop {
        let mut next_flake = (start_position.0, current_y);
        cnt += 1;

        loop {
            if next_flake.1 >= low {
                break 'outer;
            }

            if !content.contains(&(next_flake.0, next_flake.1+1)) && !snowflakes.contains(&(next_flake.0, next_flake.1+1)) {
                next_flake = (next_flake.0, next_flake.1+1);
            } else if !content.contains(&(next_flake.0-1, next_flake.1+1)) && !snowflakes.contains(&(next_flake.0-1, next_flake.1+1)) {
                next_flake = (next_flake.0-1, next_flake.1+1);
            } else if !content.contains(&(next_flake.0+1, next_flake.1+1)) && !snowflakes.contains(&(next_flake.0+1, next_flake.1+1)) {
                next_flake = (next_flake.0+1, next_flake.1+1);
            } else {
                break;
            }

        }
        if next_flake.1 == current_y {
            current_y -= 1;
        }
        println!("added {:?}", next_flake);
        snowflakes.insert(next_flake);
    }
    println!("{} {}", cnt, snowflakes.len());
}

fn main() {
    let content: HashSet<(i32, i32)> = read_from_file();
    //first_part(&content, (500,0));

    second_part(&content, (500,0));

}
