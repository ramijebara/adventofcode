use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use nalgebra::{Matrix3, Vector3};

fn main() {
    const RADIX: u32 = 10;

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        return;
    };

    let mut scanners: Vec<Scanner> = Vec::new();
    let mut scanner = 0;

    for line in data_lines {
        if let Ok(x) = line {
            if x.contains("---") {
                scanners.push(Scanner {
                    number: scanners.len(),
                    beacons: Vec::new(),
                });
                scanner = scanners.len() - 1;
            }

            if !x.contains("---") && x.len() > 0 {
                let point: Vec<&str> = x.split(",").collect::<Vec<&str>>();
                let point: Vec<i32> = point
                    .iter()
                    .map(|&n| i32::from_str_radix(n, RADIX).unwrap())
                    .collect();
                let beacon = Vector3::from_vec(point);
                scanners[scanner].beacons.push(beacon);
            }
        }
    }

    let potential_duplicate_beacons = potential_overlaps(&scanners);

    let potential_mappings = process_potential_overlap_beacons(&potential_duplicate_beacons);

    let (one_to_one, scanner_to_scanner_overlap) =
        process_valid_overlap_beacons(&scanners, &potential_mappings);

    let mut scanner_coordinates: HashMap<usize, Vector3<i32>> = HashMap::new();

    scanner_coordinates.insert(0, Vector3::from_vec(vec![0, 0, 0]));

    let mut base_scanner: usize = 0;

    loop {
        if scanner_coordinates.iter().len() == scanners.len() {
            break;
        }
        if scanner_coordinates.contains_key(&base_scanner) {
            for working_scanner in 0..scanners.len() {
                if scanner_coordinates.contains_key(&working_scanner) {
                    continue;
                }

                // check if it is a valid mapping
                // if valid rotate and translate coordinates
                if scanner_to_scanner_overlap
                    .iter()
                    .filter(|&m| *m == (base_scanner, working_scanner))
                    .count()
                    == 1
                {
                    let (transform, diff) = calculate_transform_and_diff(
                        &one_to_one,
                        &scanners,
                        base_scanner,
                        working_scanner,
                    );

                    // insert in list of known scanner  rotate and translate this scanner so it is on base
                    // scanner coordinate system (first base is scanner 0) this way all future translations
                    // will be relative to scanner 0 even if the base is the current working scanner

                    println!("adding scanner {working_scanner} to list using base {base_scanner}");
                    scanner_coordinates.insert(working_scanner, diff);
                    scanners[working_scanner].rotate_beacons(transform);
                    scanners[working_scanner].translate_coordinates(diff);
                }
            }
        }

        if base_scanner < scanners.len() {
            base_scanner += 1;
        } else {
            base_scanner = 1;
        }
    }

    println!("---------------");
    let mut sorted_keys: Vec<usize> = scanner_coordinates.keys().cloned().collect::<Vec<usize>>();
    sorted_keys.sort();
    sorted_keys.iter().for_each(|k| {
        if let Some(v) = scanner_coordinates.get(&k) {
            println!("scanner: {k}, coordinates: {v}");
        }
    });
    println!("---------------");

    let mut max_distance = 0;

    scanner_coordinates.iter().for_each(|(_, x)| {
        scanner_coordinates.iter().for_each(|(_, y)| {
            let d = manhattan_distance(x, y);
            if d > max_distance {
                max_distance = d;
            }
        });
    });

    println!("max distance: {max_distance}");
}

fn calculate_transform_and_diff(
    one_to_one: &Vec<((usize, usize), (usize, usize))>,
    scanners: &Vec<Scanner>,
    base: usize,
    other: usize,
) -> (Matrix3<i32>, Vector3<i32>) {
    let transforms: Vec<Matrix3<i32>> = get_transforms();
    let mut transform: Matrix3<i32> = Matrix3::from_vec(vec![1, 0, 0, 0, 1, 0, 0, 0, 1]);
    let mut diff = Vector3::from_vec(vec![0, 0, 0]);

    let working_set: Vec<_> = one_to_one
        .iter()
        .filter(|(sb1, sb2)| sb1.0 == base && sb2.0 == other)
        .collect();

    let xb0 = scanners[working_set[0].0 .0].get_beacon(working_set[0].0 .1);
    let yb0 = scanners[working_set[0].1 .0].get_beacon(working_set[0].1 .1);

    // just get the next set and calculate appropriate transform
    let xb1 = scanners[working_set[1].0 .0].get_beacon(working_set[1].0 .1);
    let yb1 = scanners[working_set[1].1 .0].get_beacon(working_set[1].1 .1);

    let diff1 = xb0 - xb1;

    for i in 0..transforms.len() {
        let rot_yb0 = transforms[i] * yb0;
        let rot_yb1 = transforms[i] * yb1;

        let diff2 = rot_yb0 - rot_yb1;

        if diff2 == diff1 {
            let d: Vector3<i32> = xb0 - rot_yb0;
            diff = d;
            transform = transforms[i];
            break;
        }
    }

    (transform, diff)
}

fn process_valid_overlap_beacons(
    scanners: &Vec<Scanner>,
    potential_mappings: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> (Vec<((usize, usize), (usize, usize))>, Vec<(usize, usize)>) {
    // 12 shared beacons minimum. (n(n-1)/2 or 12(12-1)/2 = 132, shared from -> to mappings)
    // no beacons on the same scanner

    let mut one_to_one = Vec::new();

    potential_mappings.iter().for_each(|(k, v)| {
        v.iter().for_each(|x| {
            one_to_one.push((k.clone(), x.clone()));
            one_to_one.push((x.clone(), k.clone()));
        });
    });

    let mut scanner_to_scanner_overlap = Vec::new();

    for current_scanner in 0..scanners.len() {
        // for each scanner find at least 12 duplicates.
        let matches: Vec<_> = one_to_one
            .iter()
            .filter(|(from, _y)| from.0 == current_scanner)
            .collect();

        for overlap in 0..scanners.len() {
            if current_scanner != overlap {
                if matches.iter().filter(|(_x, to)| to.0 == overlap).count() > 11 {
                    scanner_to_scanner_overlap.push((current_scanner, overlap));
                }
            }
        }
    }

    (one_to_one, scanner_to_scanner_overlap)
}

fn process_potential_overlap_beacons(
    potential_duplicate_beacons: &HashMap<Vec<i32>, Vec<((usize, usize), (usize, usize))>>,
) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut potential_mappings: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    potential_duplicate_beacons.iter().for_each(|(_k, v)| {
        match potential_mappings.get_mut(&v[0].0) {
            Some(m) => {
                for i in 1..v.len() {
                    m.push(v[i].0.clone());
                    m.push(v[i].1.clone());
                }
            }
            None => {
                let key = v[0].0.clone();
                let mut m = Vec::new();
                for i in 1..v.len() {
                    m.push(v[i].0.clone());
                    m.push(v[i].1.clone());
                }
                potential_mappings.insert(key, m);
            }
        }

        match potential_mappings.get_mut(&v[0].1) {
            Some(m) => {
                for i in 1..v.len() {
                    m.push(v[i].0.clone());
                    m.push(v[i].1.clone());
                }
            }
            None => {
                let key = v[0].1.clone();
                let mut m = Vec::new();
                for i in 1..v.len() {
                    m.push(v[i].0.clone());
                    m.push(v[i].1.clone());
                }
                potential_mappings.insert(key, m);
            }
        }
    });

    potential_mappings.iter_mut().for_each(|(_k, v)| {
        let mut new_v = Vec::new();
        v.iter().for_each(|x1| {
            if v.iter().filter(|&x2| x2 == x1).count() > 1 {
                new_v.push(x1.clone());
            }
        });
        new_v.sort();
        new_v.dedup();
        *v = new_v;
    });

    potential_mappings
}

fn potential_overlaps(
    scanners: &Vec<Scanner>,
) -> HashMap<Vec<i32>, Vec<((usize, usize), (usize, usize))>> {
    let mut distance_duplicates: HashMap<Vec<i32>, Vec<((usize, usize), (usize, usize))>> =
        HashMap::new();

    scanners.iter().for_each(|s| {
        s.beacons.iter().enumerate().for_each(|(i, b1)| {
            s.beacons.iter().enumerate().for_each(|(j, b2)| {
                if i != j {
                    let diff = b1 - b2;
                    let mut dist = vec![diff[0].abs(), diff[1].abs(), diff[2].abs()];
                    dist.sort();

                    match distance_duplicates.get_mut(&dist) {
                        Some(dups) => {
                            if !dups.contains(&((s.number, i), (s.number, j)))
                                && !dups.contains(&((s.number, j), (s.number, i)))
                            {
                                dups.push(((s.number, i), (s.number, j)));
                            }
                        }
                        None => {
                            let mut dups = Vec::new();
                            dups.push(((s.number, i), (s.number, j)));
                            distance_duplicates.insert(dist, dups);
                        }
                    }
                }
            });
        });
    });

    let mut to_remove: Vec<Vec<i32>> = Vec::new();

    distance_duplicates.iter_mut().for_each(|(k, v)| {
        if v.len() < 2 {
            to_remove.push(k.clone());
        }
    });

    to_remove.iter().for_each(|k| {
        distance_duplicates.remove(k).unwrap();
    });

    distance_duplicates
}

fn manhattan_distance(vec1: &Vector3<i32>, vec2: &Vector3<i32>) -> i32 {
    let diff = vec1 - vec2;
    diff[0].abs() + diff[1].abs() + diff[2].abs()
}

fn get_transforms() -> Vec<Matrix3<i32>> {
    // Return a list of possible matrix transforms
    // matrix 90 degree transforms in 3d space https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm

    let mut rotation_transforms: Vec<Matrix3<i32>> = Vec::new();
    rotation_transforms.push(Matrix3::from_vec(vec![1, 0, 0, 0, 1, 0, 0, 0, 1]));
    rotation_transforms.push(Matrix3::from_vec(vec![1, 0, 0, 0, 0, -1, 0, 1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![1, 0, 0, 0, -1, 0, 0, 0, -1]));
    rotation_transforms.push(Matrix3::from_vec(vec![1, 0, 0, 0, 0, 1, 0, -1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, -1, 0, 1, 0, 0, 0, 0, 1]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, 1, 1, 0, 0, 0, 1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 1, 0, 1, 0, 0, 0, 0, -1]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, -1, 1, 0, 0, 0, -1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![-1, 0, 0, 0, -1, 0, 0, 0, 1]));
    rotation_transforms.push(Matrix3::from_vec(vec![-1, 0, 0, 0, 0, -1, 0, -1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![-1, 0, 0, 0, 1, 0, 0, 0, -1]));
    rotation_transforms.push(Matrix3::from_vec(vec![-1, 0, 0, 0, 0, 1, 0, 1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 1, 0, -1, 0, 0, 0, 0, 1]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, 1, -1, 0, 0, 0, -1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, -1, 0, -1, 0, 0, 0, 0, -1]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, -1, -1, 0, 0, 0, 1, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, -1, 0, 1, 0, 1, 0, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 1, 0, 0, 0, 1, 1, 0, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, 1, 0, -1, 0, 1, 0, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, -1, 0, 0, 0, -1, 1, 0, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, -1, 0, -1, 0, -1, 0, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, -1, 0, 0, 0, 1, -1, 0, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 0, 1, 0, 1, 0, -1, 0, 0]));
    rotation_transforms.push(Matrix3::from_vec(vec![0, 1, 0, 0, 0, -1, -1, 0, 0]));

    rotation_transforms
}

#[derive(Debug, Clone)]
struct Scanner {
    number: usize,
    beacons: Vec<Vector3<i32>>,
}

impl Scanner {
    pub fn get_beacon(&self, beacon: usize) -> Vector3<i32> {
        self.beacons[beacon]
    }

    pub fn rotate_beacons(&mut self, transform: Matrix3<i32>) {
        let mut new_beacons: Vec<Vector3<i32>> = Vec::new();

        self.beacons.iter().for_each(|b| {
            let rot_b = transform * b;
            new_beacons.push(rot_b);
        });

        self.beacons = new_beacons;
    }

    pub fn translate_coordinates(&mut self, diff: Vector3<i32>) {
        let mut new_beacons: Vec<Vector3<i32>> = Vec::new();

        self.beacons.iter().for_each(|b| {
            let new_b = diff + b;
            new_beacons.push(new_b);
        });

        self.beacons = new_beacons;
    }
}
