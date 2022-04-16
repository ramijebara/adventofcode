use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
};

fn main() {
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

    let mut on_cubiods: Vec<Cuboid> = Vec::new();

    // reverse to create a stack
    operations.reverse();

    loop {
        if operations.len() == 0 {
            break;
        }

        // pop an operation frm the stack
        let cur_op = operations.pop().unwrap();

        // handle special case of first on operation
        if on_cubiods.len() == 0 && cur_op.operation == "on".to_string() {
            on_cubiods.push(cur_op);
            continue;
        }

        // intersect and process current operation with previous ones to
        // figure out which cubes to keep on

        on_cubiods.reverse();
        let mut x_list: Vec<Cuboid> = Vec::new();

        loop {
            if on_cubiods.len() == 0 {
                break;
            }

            let prev_op = on_cubiods.pop().unwrap();

            if prev_op.intersect(&cur_op) {
                // need to split up previous op to remove overlap.

                if prev_op.x_min > cur_op.x_min {
                    println!("prev_op x_max");
                    //new range start with x_max + 1
                    // previous x_min..cur x_max
                }

                if prev_op.y_min > cur_op.y_min {
                    println!("prev_op y_max");
                }

                if prev_op.z_min > cur_op.z_min {
                    println!("prev_op z_max\n");
                }
            } else {
                // prev_op is guaranteed to be on
                // if it does not intersect with cur_op
                // just add it back
                x_list.push(prev_op);
            }
        }

        if cur_op.operation == "on".to_string() {
            x_list.push(cur_op);
        }

        on_cubiods = x_list;
    }

    // print resultant on cubes
    on_cubiods.iter().for_each(|c| println!("{c}"));
}

#[derive(Debug, Clone)]
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
