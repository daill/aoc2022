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
                                    let mut ys = (rock.1..next_rock.1);
                                    if rock.1 > next_rock.1{
                                        ys = (next_rock.1..rock.1);
                                    }
                                    paths.extend(ys.map(|y| (rock.0, y)).collect::<HashSet<(i32,i32)>>());
                                } else {
                                    let mut xs = (rock.0..next_rock.0);
                                    if rock.0 > next_rock.0{
                                        xs = (next_rock.0..rock.0);
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
    // check where the end is
    if let Some(id) = content.iter().find(|(x,_)| x == &start_position.0) {
        println!("{:?}", id);
        bottom = id.clone();
    }

    let left = content.iter().min_by(|e, f| e.0.cmp(&f.0)).unwrap().0;
    let right = content.iter().max_by(|e, f| e.0.cmp(&f.0)).unwrap().0;

    // start flake
    let mut current_y = bottom.1-1;
    let mut it = (start_position.0, current_y);
    snowflakes.insert(it);

    loop {
        let mut next_flake = (start_position.0, current_y-1);
        let mut cycle = 1;
        let mut try_right = false;
        if content.contains(&(next_flake.0-1, next_flake.1)) {
            // left bound rock go check right
        } else {
            loop {
                if !try_right {
                    let l = &(next_flake.0 - cycle, next_flake.1 + cycle);
                    if snowflakes.contains(l) || content.contains(l) {
                        if cycle == 1 {
                            // try right
                            try_right = true;
                        } else {
                            println!("{:?}", &(next_flake.0 - cycle + 1, next_flake.1 + cycle - 1));
                            snowflakes.insert((next_flake.0 - cycle + 1, next_flake.1 + cycle - 1));
                            cycle = 1;
                            break;
                        }
                    } else {
                        cycle += 1;
                    }
                } else {
                    let r = &(next_flake.0 + cycle, next_flake.1 + cycle);
                    if snowflakes.contains(r) || content.contains(r) {
                        if cycle == 1 {
                            current_y -= 1;

                        } else {
                            println!("{:?}", &(next_flake.0 + cycle - 1, next_flake.1 + cycle - 1));
                            snowflakes.insert((next_flake.0 + cycle - 1, next_flake.1 + cycle - 1));
                            cycle = 1;
                            break;
                        }
                    } else {
                        cycle += 1;
                    }
                }
            }
        }
    }

}

fn main() {
    let content: HashSet<(i32, i32)> = read_from_file();
    first_part(&content, (500,0));

}
