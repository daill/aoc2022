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

#[derive(Debug, Eq, Clone)]
struct Sand {
    point: (i32, i32),
    left: Option<Box<Sand>>,
    mid: Option<Box<Sand>>,
    right: Option<Box<Sand>>,
}

impl Hash for Sand {
    fn hash<H>(&self, state: &mut H)
        where
            H: std::hash::Hasher,
    {
        state.write_i32(self.point.0);
        state.write_i32(self.point.1);
        state.finish();
    }
}

impl PartialEq for Sand {
    fn eq(&self, other: &Self) -> bool {
        self.point.0 == other.point.0 && self.point.1 == other.point.1
    }
}


impl Sand {
    fn new(x: &i32, y: &i32) -> Sand {
        Sand {
            point: (x.clone(), y.clone()),
            left: None,
            mid: None,
            right: None,
        }
    }
}

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


fn first_part(content: &HashSet<(i32, i32)>, start_position: (i32, i32)) {
    let mut snowflakes: HashSet<(i32, i32)> = HashSet::new();
    let mut bottom = (0,0);

    let high = content.iter().filter(|e| e.0 == start_position.0).min_by(|e, f| e.1.cmp(&f.1)).unwrap().1;
    let low = content.iter().max_by(|e, f| e.1.cmp(&f.1)).unwrap().1;

    // start flake
    let mut current_y = high-1;
    let mut it = (start_position.0, current_y);
    snowflakes.insert(it);
    let mut cnt = 0;

    'outer: loop {
        let mut next_flake = (start_position.0, current_y);
        let mut cycle = 1;
        let mut try_right = false;
        let mut b_offset = 0;
        cnt += 1;

        loop {
            if next_flake.1 + cycle + b_offset > low {
                break 'outer;
            }

            if !try_right {
                let l = &(next_flake.0 - cycle, next_flake.1 + cycle + b_offset);
                if snowflakes.contains(l) || content.contains(l) {
                    if cycle == 1 {
                        // try right
                        try_right = true;
                        b_offset = 0;
                    } else {
                        println!("{:?}", &(next_flake.0 - cycle + 1, next_flake.1 + cycle - 1 + b_offset));
                        snowflakes.insert((next_flake.0 - cycle + 1, next_flake.1 + cycle - 1 + b_offset));
                        cycle = 1;
                        b_offset = 0;
                        break;
                    }
                } else {
                    if !snowflakes.contains(&(next_flake.0 - cycle+1, next_flake.1 + cycle + b_offset)) && !content.contains(&(next_flake.0 - cycle+1, next_flake.1 + cycle + b_offset)) {
                        println!("no ground {:?}",&(next_flake.0 - cycle+1, next_flake.1 + cycle + b_offset));
                        b_offset += 1;
                    } else {
                        cycle += 1;
                    }
                }
            } else {

                let r = &(next_flake.0 + cycle, next_flake.1 + cycle + b_offset);
                if snowflakes.contains(r) || content.contains(r) {
                    if cycle == 1 {
                        current_y -= 1;

                        println!("{:?}", next_flake);
                        snowflakes.insert(next_flake);
                        break;
                    } else {
                        println!("{:?}", &(next_flake.0 + cycle - 1, next_flake.1 + cycle - 1));
                        snowflakes.insert((next_flake.0 + cycle - 1, next_flake.1 + cycle - 1));
                        cycle = 1;
                        b_offset = 0;
                        break;
                    }
                } else {
                    if !snowflakes.contains(&(next_flake.0 + cycle - 1, next_flake.1 + cycle + b_offset)) && !content.contains(&(next_flake.0 + cycle - 1, next_flake.1 + cycle + b_offset)) {
                        println!("no ground {:?}",&(next_flake.0 + cycle - 1, next_flake.1 + cycle + b_offset));
                        b_offset += 1;
                    } else {
                        cycle += 1;
                    }
                }
            }
        }
    }
    println!("{} {}", cnt, snowflakes.len());
}

fn main() {
    let content: HashSet<(i32, i32)> = read_from_file();
    first_part(&content, (500,0));

}
