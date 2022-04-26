use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    const OFF: &str = "off";
    const ON: &str = "on";

    let data_lines = if let Ok(file) = File::open("./src/bin/sample2.txt") {
        io::BufReader::new(file).lines()
    } else {
        return;
    };

    let mut operations: Vec<Cuboid> = Vec::new();

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

            operations.push(c);
        }
    }

    let mut compressed_operations: Vec<Cuboid> = Vec::new();

    // reverse to create a stack
    operations.reverse();

    loop {
        if operations.len() == 0 {
            break;
        }

        // pop an operation from the stack
        let cur = operations.pop().unwrap();

        // handle special case of first on operation
        if compressed_operations.len() == 0 {
            if cur.operation == ON {
                compressed_operations.push(cur);
            }
            continue;
        }

        // intersect and process current operation with previous ones to
        // figure out which cubes to keep on

        compressed_operations.reverse();
        let mut x_list: Vec<Cuboid> = Vec::new();

        // remove all the pieces that intersect with current operation
        loop {
            if compressed_operations.len() == 0 {
                break;
            }

            let prev = compressed_operations.pop().unwrap();

            // if there is no intersection push previous operation
            // and continue
            if !prev.intersect(&cur) {
                x_list.push(prev);
                continue;
            }

            if prev.operation == OFF {
                x_list.push(prev);
                continue;
            }

            // if we are at this point then we build an "off" cuboid of
            // the intersection to avoid double counting
            let i_x_min = if prev.x_min >= cur.x_min {
                prev.x_min
            } else {
                cur.x_min
            };
            let i_x_max = if prev.x_max <= cur.x_max {
                prev.x_max
            } else {
                cur.x_max
            };
            let i_y_min = if prev.y_min >= cur.y_min {
                prev.y_min
            } else {
                cur.y_min
            };
            let i_y_max = if prev.y_max <= cur.y_max {
                prev.y_max
            } else {
                cur.y_max
            };
            let i_z_min = if prev.z_min >= cur.z_min {
                prev.z_min
            } else {
                cur.z_min
            };
            let i_z_max = if prev.z_max <= cur.z_max {
                prev.z_max
            } else {
                cur.z_max
            };

            let c = Cuboid::new(
                OFF.to_string(),
                i_x_min,
                i_x_max,
                i_y_min,
                i_y_max,
                i_z_min,
                i_z_max,
            );
            x_list.push(prev);
            x_list.push(c);
        }

        // add current operation to to list if on
        if cur.operation == ON {
            x_list.push(cur);
        }

        // reassign the list back to on_cubiods
        compressed_operations = x_list;
    }

    // print result
    let mut result: isize = 0;
    compressed_operations.iter().for_each(|c| {
        if c.operation == ON {
            result += c.num_of_cubes();
        } else {
            //result -= c.num_of_cubes();
        }
        println!("{c}");
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
    pub fn intersect(&self, c: &Cuboid) -> bool {
        self.x_min <= c.x_max
            && self.y_min <= c.y_max
            && self.z_min <= c.z_max
            && c.x_min <= self.x_max
            && c.y_min <= self.y_max
            && c.z_min <= self.z_max
    }

    // calculate size
    pub fn num_of_cubes(&self) -> isize {
        ((self.x_max - self.x_min) * (self.y_max - self.y_min) * (self.z_max - self.z_min)).abs()
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
