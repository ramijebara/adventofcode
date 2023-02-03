use std::{
    fs::File,
    io::{self, BufRead},
};

use log::{error, info};

fn main() {
    env_logger::init();

    let mut data_lines = if let Ok(file) = File::open("src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let light_row: Vec<usize> = vec![0; 1000];
    let mut light_matrix: Vec<Vec<usize>> = vec![light_row; 1000];

    while let Some(Ok(instruction)) = data_lines.next() {
        info!("{instruction}");
        perform_action(instruction, &mut light_matrix);
    }

    let count: _ = light_matrix.iter().map(|x| { x.iter().sum::<usize>() }).sum::<usize>();
    println!("Result: {}", count);
}

fn turn_on(x: usize, y: usize, light_matrix: &mut Vec<Vec<usize>>) {
    light_matrix[y][x] = 1;
}

fn turn_off(x: usize, y: usize, light_matrix: &mut Vec<Vec<usize>>) {
    light_matrix[y][x] = 0;
}

fn toggle(x: usize, y: usize, light_matrix: &mut Vec<Vec<usize>>) {
    if light_matrix[y][x] == 0 {
        light_matrix[y][x] = 1;
    } else {
        light_matrix[y][x] = 0;
    }
}

fn perform_action(instruction: String, light_matrix: &mut Vec<Vec<usize>>) {
    let instruction_parts: Vec<&str> = instruction.split(' ').collect();

    if instruction_parts[0] == "turn" {
        if instruction_parts[1] == "on" {
            let start_coords: Vec<usize> = instruction_parts[2]
                .split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let end_coords: Vec<usize> = instruction_parts[4]
                .split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            for y in start_coords[1]..(end_coords[1] + 1) {
                for x in start_coords[0]..(end_coords[0] + 1) {
                    turn_on(x, y, light_matrix);
                }
            }
        } else if instruction_parts[1] == "off" {
            let start_coords: Vec<usize> = instruction_parts[2]
                .split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let end_coords: Vec<usize> = instruction_parts[4]
                .split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            for y in start_coords[1]..(end_coords[1] + 1) {
                for x in start_coords[0]..(end_coords[0] + 1) {
                    turn_off(x, y, light_matrix);
                }
            }
        } else {
            error!("Bad instruction on line: {}", instruction);
        }
    } else if instruction_parts[0] == "toggle" {
        let start_coords: Vec<usize> = instruction_parts[1]
            .split(',')
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let end_coords: Vec<usize> = instruction_parts[3]
            .split(',')
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for y in start_coords[1]..(end_coords[1] + 1) {
            for x in start_coords[0]..(end_coords[0] + 1) {
                toggle(x, y, light_matrix);
            }
        }
    } else {
        error!("Bad instruction on line: {}", instruction);
    }
}
