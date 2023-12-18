use std::{fs::File, io::BufRead, io::BufReader};
use color_eyre::eyre::Result;
use log::{trace, info};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day3/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut result = 0;

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("input: {}", data_line);
        data.push(data_line.chars().collect::<Vec<char>>());
    }

    for row in 0..data.len() {
        let mut number = String::new();
        let mut coords = Vec::new();
        for col in 0..data[row].len() {
            if data[row][col].is_digit(10) {
                number.push(data[row][col]);
                coords.push((row, col));
            } else {
                if number.len() > 0 {
                    let parsed_number = number.parse::<usize>()?;
                    trace!("{}: {:?}", parsed_number, coords);
                    if is_countable(coords, &data) {
                        result += parsed_number;
                    }
                    number = String::new();
                    coords = Vec::new();
                }
            }
        }
        // end of line edge case
        if number.len() > 0 {
            let parsed_number = number.parse::<usize>()?;
            trace!("{}: {:?}", parsed_number, coords);
            if is_countable(coords, &data) {
                result += parsed_number;
            }
        }
    }
    info!("result: {}", result);
    Ok(())
}

fn is_countable(coords: Vec<(usize, usize)>, data: &Vec<Vec<char>>) -> bool {
    let row_max = data.len() - 1;
    let col_max = if row_max > 0 { data[0].len() - 1 } else { 0 };
    let non_symbols: Vec<char> = ".0123456789".chars().collect::<Vec<char>>();
    
    for (row, col) in coords {
        if row > 0 && col > 0 && is_symbol(&data[row - 1][col - 1], &non_symbols) {
            return true;
        }
        if col > 0 && is_symbol(&data[row][col - 1], &non_symbols) {
            return true;
        }
        if col < col_max && is_symbol(&data[row][col + 1], &non_symbols) {
            return true;
        }
        if row > 0 && is_symbol(&data[row - 1][col], &non_symbols) {
            return true;
        }
        if row < row_max && is_symbol(&data[row + 1][col], &non_symbols) {
            return true;
        }
        if row < row_max && col < col_max && is_symbol(&data[row + 1][col + 1], &non_symbols) {
            return true;
        }
        if row > 0 && col < col_max && is_symbol(&data[row - 1][col + 1], &non_symbols) {
            return true;
        }
        if row < row_max && col > 0 && is_symbol(&data[row + 1][col - 1], &non_symbols) {
            return true;
        }
    }

    false
}

fn is_symbol(c: &char, non_symbols: &Vec<char>) -> bool {
    !non_symbols.contains(c)
} 
