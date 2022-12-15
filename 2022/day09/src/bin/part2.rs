use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use log::{error, info};

fn main() {
    env_logger::init();
    const ROPE_LENGTH: usize = 10;

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); ROPE_LENGTH];
    let mut visit_coords: HashSet<(i32, i32)> = HashSet::new();

    for line in data_lines {
        if let Ok(x) = line {
            let instruction: Vec<&str> = x.split(" ").collect();

            let direction = instruction[0];
            let steps = instruction[1].parse::<i32>().unwrap();

            for _ in 0..steps {
                // insert last tail position
                visit_coords.insert(rope[ROPE_LENGTH - 1]);

                // move head
                match direction {
                    "U" => rope[0].1 += 1,
                    "D" => rope[0].1 -= 1,
                    "R" => rope[0].0 += 1,
                    "L" => rope[0].0 -= 1,
                    _ => error!("Invalid position"),
                }

                for i in 1..ROPE_LENGTH {
                    info!("Head: {:?}, Tail: {:?}", rope[i - 1], rope[i]);
                    rope[i] = update_tail_position(rope[i - 1], rope[i]);
                }
            }

            // insert last position
            visit_coords.insert(rope[ROPE_LENGTH - 1]);
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
