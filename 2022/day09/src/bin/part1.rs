use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use log::{error, info};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut head_position: (i32, i32) = (0, 0); // (x, y)
    let mut tail_position: (i32, i32) = (0, 0); // (x, y)
    let mut visit_coords: HashSet<(i32, i32)> = HashSet::new();

    for line in data_lines {
        if let Ok(x) = line {
            let instruction: Vec<&str> = x.split(" ").collect();

            let direction = instruction[0];
            let steps = instruction[1].parse::<i32>().unwrap();

            for _ in 0..steps {
                visit_coords.insert(tail_position);

                info!("Head: {:?}, Tail: {:?}", head_position, tail_position);

                match direction {
                    "U" => head_position.1 += 1,
                    "D" => head_position.1 -= 1,
                    "R" => head_position.0 += 1,
                    "L" => head_position.0 -= 1,
                    _ => error!("Invalid position"),
                }

                tail_position = update_tail_position(head_position, tail_position);
            }

            // insert last position
            visit_coords.insert(tail_position);
        }
    }

    println!("Result: {}", visit_coords.len());
}

fn update_tail_position(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    // move to right
    if (h.0 - t.0) > 1 && (h.1 - t.1) == 0 {
        return (t.0 + 1, t.1);
    }

    // move to left
    if (h.0 - t.0) < -1 && (h.1 - t.1) == 0 {
        return (t.0 - 1, t.1);
    }

    // move up
    if (h.0 - t.0) == 0 && (h.1 - t.1) > 1 {
        return (t.0, t.1 + 1);
    }

    // move down
    if (h.0 - t.0) == 0 && (h.1 - t.1) < -1 {
        return (t.0, t.1 - 1);
    }

    // diagonal up to the right
    if ((h.0 - t.0) > 0 && (h.1 - t.1) > 1) || ((h.0 - t.0) > 1 && (h.1 - t.1) > 0) {
        return (t.0 + 1, t.1 + 1);
    }

    // diagonal up to the left
    if ((h.0 - t.0) < 0 && (h.1 - t.1) > 1) || ((h.0 - t.0) < -1 && (h.1 - t.1) > 0) {
        return (t.0 - 1, t.1 + 1);
    }

    // diagonal down to the right
    if ((h.0 - t.0) > 0 && (h.1 - t.1) < -1) || ((h.0 - t.0) > 1 && (h.1 - t.1) < 0) {
        return (t.0 + 1, t.1 - 1);
    }

    // diagonal down to the left
    if ((h.0 - t.0) < 0 && (h.1 - t.1) < -1) || ((h.0 - t.0) < -1 && (h.1 - t.1) < 0) {
        return (t.0 - 1, t.1 - 1);
    }

    // no movement return t
    t
}
