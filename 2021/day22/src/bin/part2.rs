/*
Theory: https://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle
Part of my solution was inspired by Jellycious's solution to this puzzle.
Thanks!
*/
use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    const ON: &str = "on";

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        return;
    };

    let mut instructions: Vec<Cuboid> = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            let operation_details: Vec<&str> = x.split(" ").collect();

            let operation = operation_details[0].to_string();

            let details: Vec<&str> = operation_details[1].split(",").collect();

            let mut x_min: isize = isize::MIN;
            let mut x_max: isize = isize::MIN;
            let mut y_min: isize = isize::MIN;
            let mut y_max: isize = isize::MIN;
            let mut z_min: isize = isize::MIN;
            let mut z_max: isize = isize::MIN;

            // parse the details into x, y and z ranges
            details.iter().for_each(|&r| {
                let unparsed_range: Vec<&str> = r.split("=").collect();

                let parsed_range: Vec<isize> = unparsed_range[1]
                    .split("..")
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect();

                match unparsed_range[0] {
                    "x" => {
                        x_min = parsed_range[0];
                        x_max = parsed_range[1];
                    }
                    "y" => {
                        y_min = parsed_range[0];
                        y_max = parsed_range[1];
                    }
                    "z" => {
                        z_min = parsed_range[0];
                        z_max = parsed_range[1];
                    }
                    _ => {}
                }
            });

            let c = Cuboid::new(operation, x_min, x_max, y_min, y_max, z_min, z_max);

            instructions.push(c);
        }
    }

    // (cuboid, add or subtract)
    // add = 1, subtract = -1
    let mut processed_instructions: Vec<(Cuboid, isize)> = Vec::new();

    for instruction in instructions {
        // create temp list to hold intersections and on cubes
        let mut temp_list: Vec<(Cuboid, isize)> = Vec::new();

        // check intersections with processed instructions
        for processed_instruction in &processed_instructions {
            if let Some(x) = instruction.intersect(&processed_instruction.0) {
                let i = (x, processed_instruction.1 * -1);
                temp_list.push(i);
            } else {
                continue;
            }
        }

        if instruction.operation == ON {
            temp_list.push((instruction, 1));
        }

        processed_instructions.append(&mut temp_list);
    }

    // print result
    let mut result: isize = 0;

    processed_instructions.iter().for_each(|(c, multiplier)| {
        result += c.num_of_cubes() * multiplier;
    });

    println!("Result: {}", result);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cuboid {
    operation: String,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Cuboid {
    pub fn new(
        operation: String,
        x_min: isize,
        x_max: isize,
        y_min: isize,
        y_max: isize,
        z_min: isize,
        z_max: isize,
    ) -> Self {
        Cuboid {
            operation,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    // Two cuboids intersect when the minimum coordinate of one
    // is smaller than the maximum of the other for all axis
    pub fn intersect(&self, other: &Cuboid) -> Option<Self> {
        if self.x_min <= other.x_max
            && self.y_min <= other.y_max
            && self.z_min <= other.z_max
            && other.x_min <= self.x_max
            && other.y_min <= self.y_max
            && other.z_min <= self.z_max
        {
            let x_min = isize::max(self.x_min, other.x_min);
            let x_max = isize::min(self.x_max, other.x_max);
            let y_min = isize::max(self.y_min, other.y_min);
            let y_max = isize::min(self.y_max, other.y_max);
            let z_min = isize::max(self.z_min, other.z_min);
            let z_max = isize::min(self.z_max, other.z_max);

            Some(Cuboid::new(
                "on".to_string(), // This is hardcoded. Can be refactored to be better
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            ))
        } else {
            None
        }
    }

    // calculate size
    pub fn num_of_cubes(&self) -> isize {
        (self.x_max - self.x_min + 1)
            * (self.y_max - self.y_min + 1)
            * (self.z_max - self.z_min + 1)
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "operation: {}, x: {}..{}, y: {}..{}, z: {}..{}",
            self.operation, self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max
        )
    }
}
