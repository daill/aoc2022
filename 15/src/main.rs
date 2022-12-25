use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug, Clone)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    dist: i32,
}

impl Sensor {
    fn new(pos: (i32, i32), beacon: (i32, i32)) -> Sensor {
        Sensor {
            pos,
            beacon,
            dist: Sensor::dist(&pos, &beacon)
        }
    }

    fn dist(src: &(i32, i32), dst: &(i32, i32)) -> i32 {
        return (src.0-dst.0).abs()+(src.1-dst.1).abs()
    }
}


fn read_from_file() -> Vec<Sensor> {
    let mut file = File::open("input");
    let mut sensors:Vec<Sensor> = Vec::new();
    match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let re = Regex::new(r"x=([-]*\d+),\W+y=([-]*\d+).*?x=([-]*\d+),\W+y=([-]*\d+)").unwrap();
            for line in lines {
                if let Ok(line) = line {
                    let caps = re.captures(&line).unwrap();
                    println!("{:?}", caps);
                    let pos: (i32, i32) = (caps.get(1).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()), caps.get(2).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()));

                    let beacon: (i32, i32) = (caps.get(3).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()), caps.get(4).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()));
                    let sensor = Sensor::new(pos, beacon);
                    println!("{:?}", sensor);
                    sensors.push(sensor);
                }
            }
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };

    return sensors;
}


fn first_part(content: &Vec<Sensor>) {
    let y = 10;
    for sensor in content {
        if sensor.pos.
    }
}

fn main() {
    let content: Vec<Sensor> = read_from_file();
    first_part(&content);

}
