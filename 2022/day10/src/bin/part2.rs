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
    let mut crt: Vec<Vec<char>> = vec![
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
    ];

    for line in data_lines {
        if let Ok(x) = line {
            let sprite = vec![x_register - 1, x_register, x_register + 1];
            info!("Sprite: {:?}", sprite);

            let mut op_cycles = 1;
            let mut x_adder = 0;

            if x.starts_with("addx") {
                let instruction = x.split(" ").collect::<Vec<_>>();
                x_adder = instruction[1].parse::<i32>().unwrap();
                op_cycles = 2;
            }

            trace!("Op cycles: {}, X adder: {}", op_cycles, x_adder);

            for _ in 0..op_cycles {
                let v_position = get_v_position(cycle);

                let h_position = cycle - STEP * (v_position as i32);
                info!("V Position: {}, H Position: {}", v_position, h_position);

                if sprite.contains(&h_position) {
                    crt[v_position][h_position as usize] = '#';
                }

                cycle += 1;
            }
            x_register += x_adder;
            info!("Cycle: {}, X Register: {}", cycle, x_register);
        }
    }
    crt.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{c}"));
        println!();
    });
}

fn get_v_position(cycle: i32) -> usize {
    if cycle >= 0 && cycle < 40 {
        return 0;
    }
    if cycle >= 40 && cycle < 80 {
        return 1;
    }
    if cycle >= 80 && cycle < 120 {
        return 2;
    }
    if cycle >= 120 && cycle < 160 {
        return 3;
    }
    if cycle >= 160 && cycle < 200 {
        return 4;
    }
    if cycle >= 200 && cycle < 240 {
        return 5;
    }

    0
}
