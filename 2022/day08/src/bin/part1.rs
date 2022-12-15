use log::{error, info};
use std::{
    collections::HashSet,
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

    let mut visiable_trees: HashSet<(usize, usize)> = HashSet::new();

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

            // edges
            if x == 0 || y == 0 || x == (width - 1) || y == (height - 1) {
                visiable_trees.insert((y, x));
            }
            // others
            else {
                let row_visible = visible(cur_value, x, &cur_row);
                let col_visible = visible(cur_value, y, &cur_col);

                if !row_visible && !col_visible {
                    info!("Not visible: ({}, {}), {}", y, x, grid[y][x]);
                } else {
                    visiable_trees.insert((y, x));
                }
            }
        }
    }

    println!("{}", visiable_trees.len());
}

fn visible(value: u32, index: usize, row_or_column: &Vec<u32>) -> bool {
    let mut side_a_visible = true;
    let mut side_b_visible = true;

    // side A
    for i in (0..index).rev() {
        if row_or_column[i] >= value {
            side_a_visible = false;
            break;
        }
    }

    // side B
    for i in index + 1..row_or_column.len() {
        if row_or_column[i] >= value {
            side_b_visible = false;
            break;
        }
    }

    if !side_a_visible && !side_b_visible {
        return false;
    }

    true
}
