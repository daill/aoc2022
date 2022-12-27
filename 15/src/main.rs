use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ptr::swap_nonoverlapping;
use regex::Regex;

#[derive(Debug, Clone)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    dist: i32,
}


impl Sensor {
    fn new(pos: (i32, i32), beacon: (i32, i32)) -> Sensor {
        let dist = Sensor::dist(&pos, &beacon);
        Sensor {
            pos,
            beacon,
            dist
        }
    }

    fn dist(src: &(i32, i32), dst: &(i32, i32)) -> i32 {
        return (src.0-dst.0).abs()+(src.1-dst.1).abs()
    }
}


fn read_from_file() -> (Vec<Sensor>, Vec<(i32, i32)>, i32, i32) {
    let mut file = File::open("input");
    let mut sensors:Vec<Sensor> = Vec::new();
    let mut beacons:Vec<(i32, i32)> = Vec::new();
    let mut min = 0;
    let mut max = 0;
    match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let re = Regex::new(r"x=([-]*\d+),\W+y=([-]*\d+).*?x=([-]*\d+),\W+y=([-]*\d+)").unwrap();
            for line in lines {
                if let Ok(line) = line {
                    let caps = re.captures(&line).unwrap();
                    let pos: (i32, i32) = (caps.get(1).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()), caps.get(2).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()));
                    let beacon= (caps.get(3).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()), caps.get(4).map_or(0 as i32, |e| e.as_str().parse::<i32>().unwrap()));
                    beacons.push(beacon);
                    if pos.0 < min || beacon.0 < min {
                        min = pos.0.min(beacon.0);
                    }
                    if pos.0 > max || beacon.0 > max{
                        max = pos.0.max(beacon.0);
                    }
                    let sensor = Sensor::new(pos, beacon);
                    sensors.push(sensor);
                }
            }
        },
        Err(e) => panic!("Cannot process file: {}", e)
    };

    return (sensors, beacons, min, max);
}


fn first_part(content: &(Vec<Sensor>, Vec<(i32, i32)>, i32, i32)) {

    let y = 2000000;
    let sensors = &content.0;
    let beacons = &content.1;
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let min = content.2;
    let max = content.3;

    //let selected = sensors.iter().filter(|sensor| ((sensor.pos.1 - sensor.dist)..(sensor.pos.1 + sensor.dist)).contains(&y)).collect::<Vec<&Sensor>>();

    sensors.iter().filter(|sensor| ((sensor.pos.1 - sensor.dist)..(sensor.pos.1 + sensor.dist)).contains(&y)).for_each( |sensor| {
        let diff = sensor.dist - (sensor.pos.1 - y).abs();
        for i in -diff..diff+1 {
            //println!("{} {:?} {:?} {}", i, sensor, &(sensor.pos.0 + i, y), Sensor::dist(&sensor.pos, &(sensor.pos.0 + i, y)));
            if Sensor::dist(&sensor.pos, &(sensor.pos.0 + i, y)) <= sensor.dist {
                if !beacons.contains(&(sensor.pos.0 + i, y)) {
                    positions.insert((sensor.pos.0 + i, y));
                }
            }
        }
    });

    println!("{}", positions.len() );
}

fn main() {
    let content: (Vec<Sensor>, Vec<(i32, i32)>, i32, i32) = read_from_file();
    first_part(&content);

}
