use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    const RADIX: u32 = 10;

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        return;
    };

    let mut data: Vec<Vec<Point>> = Vec::new();
    let mut beacon_num = 0;

    for line in data_lines {
        if let Ok(x) = line {
            if x.contains("---") {
                data.push(Vec::new());
                beacon_num = data.len() - 1;
            }

            if !x.contains("---") && x.len() > 0 {
                let point: Vec<&str> = x.split(",").collect::<Vec<&str>>();
                let point: Vec<i32> = point
                    .iter()
                    .map(|&n| i32::from_str_radix(n, RADIX).unwrap())
                    .collect();
                let beacon_point = Point::new(point[0], point[1], point[2]);
                data[beacon_num].push(beacon_point);
            }
        }
    }

    let mut sorted_distances: HashMap<(i32, i32, i32), Vec<(usize, usize)>> = HashMap::new();
    let mut scanner_beacons: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            // create a list of scanner beacon combos
            scanner_beacons.insert((i, j), Vec::new());

            for k in 0..data[i].len() {
                if j != k {
                    let mut delta = data[i][j].delta_abs(&data[i][k]);
                    delta.sort();

                    let key = (delta[0], delta[1], delta[2]);
                    let s_p1 = (i, j);
                    let s_p2 = (i, k);

                    if sorted_distances.contains_key(&key) {
                        if let Some(v) = sorted_distances.get_mut(&key) {
                            if !v.contains(&s_p1) {
                                v.push(s_p1);
                            }
                            if !v.contains(&s_p2) {
                                v.push(s_p2);
                            }
                        }
                    } else {
                        sorted_distances.insert(key, Vec::from([s_p1, s_p2]));
                    }
                }
            }
        }
    }

    for (_k, v) in sorted_distances {
        if v.len() > 2 {
            if let Some(dups) = scanner_beacons.get_mut(&v[0]) {
                if dups.len() == 0 {
                    dups.append(&mut v[2..v.len()].to_vec());
                } else {
                    for i in 2..v.len() {
                        // insert if new scanner
                        if !dups.contains(&v[i]) {
                            dups.push(v[i]);
                        }
                    }
                }
            }

            if let Some(dups) = scanner_beacons.get_mut(&v[1]) {
                if dups.len() == 0 {
                    dups.append(&mut v[2..v.len()].to_vec());
                } else {
                    for i in 2..v.len() {
                        // insert if new scanner
                        if !dups.contains(&v[i]) {
                            dups.push(v[i]);
                        }
                    }
                }
            }
        }
    }

    let mut to_delete: Vec<(usize, usize)> = Vec::new();

    for (_, v) in &scanner_beacons {
        if v.len() > 0 {
            for s_p in v {
                if !to_delete.contains(&s_p) {
                    to_delete.push(*s_p);
                }
            }
        }
    }

    for x in to_delete {
        scanner_beacons.remove(&x).unwrap();
    }

    println!("number of beacons: {}", scanner_beacons.len());
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    pub fn delta_abs(&self, point: &Point) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        let dx = self.x - point.x;
        res.push(dx.abs());
        let dy = self.y - point.y;
        res.push(dy.abs());
        let dz = self.z - point.z;
        res.push(dz.abs());

        res
    }
}
