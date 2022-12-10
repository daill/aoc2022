use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use std::slice::RSplit;
use std::str::Chars;



fn read_from_file() -> (u32, u32, Vec<u32>) {
    let mut file = File::open("input");
    let mut content: Vec<u32> = Vec::new();
    let mut length = 0;
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {


                if let Ok(line) = line {
                    println!("line {}", line);
                    content.append(&mut line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
                    if length == 0 {
                        length = content.len() as u32;
                    }
                }
            }
            println!("content {:?}", content);
            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    return (length, (content.len() as u32/length) as u32, content);
}


fn first_part(length: &u32, height: &u32, content: &Vec<u32>)  {
    let mut sum = length*2+(height*2-4);
    for x in 1..*length-1 {
        let rng = (x..content.len() as u32).step_by(usize::try_from(*length).unwrap()).collect::<Vec<u32>>();

        let col = content.iter().enumerate().filter(|e| rng.contains(&(e.0 as u32))).map(|e| e.1).collect::<Vec<&u32>>();

        for y in 1..*height-1 {
            let val = content.get((x+y*length) as usize).unwrap();
            //println!("x {} y {} v {}", x, y, val);
            // l and r
            let l = (y*length) as usize;
            let r = ((y+1)*length) as usize;
            let (row_r, row_l) = content[l..r].split_at(x as usize);
            let (col_u, col_l) = col.split_at(y as usize);
            let lrc = row_l.iter().filter(|e| e >= &val).count()-1;
            let rrc= row_r.iter().filter(|e| e >= &val).count();
            let ucc = col_u.iter().filter(|e| e >= &&val).count();
            let lcc = col_l.iter().filter(|e| e >= &&val).count()-1;
            //println!("{:?} {:?} {:?} {:?}", row_r, row_l, col_u, col_l);

            //println!("{} {} {} {}", rrc, lrc, ucc, lcc);
             if lrc == 0 || rrc == 0 || ucc == 0 || lcc == 0 {
                 sum +=1;
             }

        }
    }
    println!("{}", sum);

}

fn second_part(length: &u32, height: &u32, content: &Vec<u32>)  {
    let mut view = 0;
    for x in 0..*length {
        let rng = (x..content.len() as u32).step_by(usize::try_from(*length).unwrap()).collect::<Vec<u32>>();

        let col = content.iter().enumerate().filter(|e| rng.contains(&(e.0 as u32))).map(|e| e.1).collect::<Vec<&u32>>();

        for y in 0..*height {
            let val = content.get((x+y*length) as usize).unwrap();
            println!("x {} y {} v {}", x, y, val);
            // l and r
            let l = (y*length) as usize;
            let r = ((y+1)*length) as usize;
            let (row_r, row_l) = content[l..r].split_at(x as usize);
            let (mut col_u, col_l) = col.split_at(y as usize);
            let mut view_count = 0;

            let mut rrc = 0;
            for  e in row_r.iter().rev() {
                rrc += 1;
                if e >= &val {
                    break;
                }
            };
            let mut lrc= 0;
            for e in row_l.iter().skip(1) {
                lrc += 1;
                if e >= &val {
                    break;
                }
            }

            let mut ucc = 0;
            for e in col_u.iter().rev() {
                ucc += 1;
                if e >= &val {
                    break;
                }
            }
            let mut lcc = 0;
            for e in col_l.iter().skip(1)  {
                lcc += 1;
                if e >= &val {
                    break;
                }
            }
            println!("{:?} {:?} {:?} {:?}", row_r, row_l, col_u, col_l);
            println!("{} {} {} {}", rrc, lrc, ucc, lcc);
            view_count = rrc * lrc * ucc * lcc;
            if view_count > view {
                view = view_count;
            }

        }
    }
    println!("{}", view);

}


fn main() {
    let (length, height, content) = read_from_file();
    println!("length {}, height {}, content {:?}", length, height, &content);

    //first_part(&length, &height, &content);

    second_part(&length, &height, &content);




}
