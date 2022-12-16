use log::{error, info, trace};
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    const STEP: i32 = 40;

    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut x_register = 1;
    let mut cycle = 0;
    let mut signal_strength_marker = 20;
    let mut result = 0;

    for line in data_lines {
        if let Ok(x) = line {
            trace!("{}", x);

            let mut op_cycles = 1;
            let mut x_adder = 0;

            if x.starts_with("addx") {
                let instruction = x.split(" ").collect::<Vec<_>>();
                x_adder = instruction[1].parse::<i32>().unwrap();
                op_cycles = 2;
            }

            trace!("Op cycles: {}, X adder: {}", op_cycles, x_adder);

            for _ in 0..op_cycles {
                cycle += 1;
                if cycle == signal_strength_marker {
                    signal_strength_marker += STEP;
                    result += cycle * x_register;
                    info!("Cycle: {}, X Register: {}", cycle, x_register);
                }
            }

            x_register += x_adder;
        }
    }

    println!("Result: {}", result);
}
