use std::any::Any;
use std::fs::File;
use std::{io, usize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use std::slice::RSplit;
use std::str::Chars;



fn read_from_file() -> HashMap<String, Vec<(String, String)>> {
    let mut file = File::open("input");
    let mut content: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let content = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut current_dir= String::from("");
            let mut current_path = String::from("/");
            let mut files: &mut Vec<(String, String)> = &mut Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    println!("line {}", line);
                    println!("{}", current_path);

                    if line.starts_with("$ cd") {
                        // command
                        let cd_target = &line[5..line.len()];

                        if cd_target == ".." {

                            let end = current_path.rfind('/').unwrap();
                            if current_path.len() > 2 {
                                current_path = String::from(&current_path[0..end]);
                            } else {
                                current_path = String::from("/");
                            }


                            println!("back {}", current_path);
                        } else {

                            println!("into {} {}", cd_target, current_path);
                                if current_path != "/" {
                                    current_path = format!("{}/{}", current_path, cd_target);
                                } else if cd_target != "/" {
                                    current_path += cd_target;
                                } else {
                                    current_path = String::from("/");
                                }
                                println!("new path {}", current_path);


                        }

                        /*
                        if cd_target == ".." {
                            let a = current_dir.rsplit("/").next().unwrap();
                            println!("{}", a);
                        } else {
                            if current_dir.len() > 1 {
                                current_dir = current_dir + "/" + cd_target;
                            } else {
                                current_dir = current_dir + cd_target;
                            }
                        }
                        */
                    } else if line.starts_with("$ ls"){
                        if !content.contains_key(&current_path) {
                            content.insert(current_path.to_owned(), Vec::new());
                        }
                        files = content.get_mut(&current_path).unwrap();
                    } else {
                        let mut splits = line.split(" ");
                        let first = String::from(splits.next().unwrap());
                        let second = String::from(splits.next().unwrap());
                        files.push((first, second));

                    }
                }
            }

            content
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };
    println!("content {:?}", &content);
    return content;
}

fn build_dirsizes(content: &HashMap<String, Vec<(String, String)>>, current_dir: &str, sizes: &mut HashMap<String, i32>) {
    let mut current_size: i32 = 0;
    if !content.contains_key(current_dir) {
        return;
    }
    println!("{}", current_dir);
    let entries = content.get(current_dir).unwrap();
    for (left, right) in entries{
        if left == "dir" {
            let mut path = String::new();
            if current_dir == "/" {
                path = format!("{}{}", current_dir, right);
            } else {
                path = format!("{}/{}", current_dir, right);
            }
            build_dirsizes(content, &path, sizes);
            println!("{} {} {:?}", current_dir, path, sizes);
            current_size += sizes.get(&path).unwrap_or(&0);
        } else {
            current_size += left.parse::<i32>().unwrap();
        }
    }
    sizes.insert(String::from(current_dir), current_size);
}

fn first_part(content: &HashMap<String, Vec<(String, String)>>)  {
    let mut sizes: HashMap<String, i32> = HashMap::new();
    build_dirsizes(content, "/", &mut sizes);
    println!("{:?}", sizes);
    let mut c = 0;
    sizes.iter().for_each(|size| {
       if size.1 < &100000 {
           println!("{}", size.0);
           c += size.1;
       }
    });
    println!("{}", c);


    let size_to_find = 30000000-(70000000 - sizes.get("/").unwrap());
    println!("to find {}", size_to_find);
    let mut filtered = sizes.iter().filter(|size| size.1 >= &size_to_find).collect::<Vec<(&String, &i32)>>();
    println!("{:?}", filtered);
    filtered.sort_by(|a, b| a.1.cmp(b.1));
    println!("{:?}", filtered);

}


fn main() {
    let content = read_from_file();


    first_part(&content);




}
