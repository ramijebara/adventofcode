use log::{error};
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut grid = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            let mut row = Vec::new();

            x.chars().into_iter().for_each(|h| {
                row.push(h.to_digit(10).unwrap());
            });

            grid.push(row);
        }
    }

    let mut scores = Vec::new();

    let height = grid.len();
    let width = grid[0].len();

    for y in 0..height {
        let cur_row = grid[y].clone();

        for x in 0..width {
            let cur_value = grid[y][x].clone();
            let cur_col = grid
                .iter()
                .map(|s| *s.iter().nth(x).unwrap())
                .collect::<Vec<_>>();

            // internal trees
            if !(x == 0 || y == 0 || x == (width - 1) || y == (height - 1)) {
                let row_score = score(cur_value, x, &cur_row);
                let col_score = score(cur_value, y, &cur_col);

                scores.push(row_score * col_score);
            }
        }
    }

    let max_score = scores.iter().max().unwrap();
    println!("Result: {}", max_score);
}


fn score(value: u32, index: usize, row_or_column: &Vec<u32>) -> usize {
    let mut s_a = 0;
    let mut s_b = 0;

    // side A
    for i in (0..index).rev() {
        s_a += 1;
        if row_or_column[i] >= value {
            break;
        }
    }

    // side B
    for i in index + 1 .. row_or_column.len() {
        s_b += 1;
        if row_or_column[i] >= value {
            break;
        }
    }

    s_a * s_b
}