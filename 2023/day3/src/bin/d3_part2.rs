use std::{fs::File, io::BufRead, io::BufReader, collections::HashMap};
use color_eyre::eyre::Result;
use log::{trace, info};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day3/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut star_gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
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
                    if let Some(star_coords) = is_countable(coords, &data) {
                        if star_gears.contains_key(&star_coords) {
                            let v = star_gears.get_mut(&star_coords).unwrap();
                            v.push(parsed_number); 
                        } else {
                            star_gears.insert(star_coords, [parsed_number].to_vec());
                        }
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
            if let Some(star_coords) = is_countable(coords, &data) {
                if star_gears.contains_key(&star_coords) {
                    let v = star_gears.get_mut(&star_coords).unwrap();
                    v.push(parsed_number); 
                } else {
                    star_gears.insert(star_coords, [parsed_number].to_vec());
                }
            }
        }
    }

    star_gears.iter().for_each(|(_, v)|{
        if v.len() > 1 {
            result += v.iter().fold(1, |p, x|{ p*x });
        }
    });

    info!("result: {}", result);
    Ok(())
}

fn is_countable(coords: Vec<(usize, usize)>, data: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let row_max = data.len() - 1;
    let col_max = if row_max > 0 { data[0].len() - 1 } else { 0 };
    
    for (row, col) in coords {
        if row > 0 && col > 0 && is_symbol(&data[row - 1][col - 1]) {
            return Some((row - 1, col - 1));
        }
        if col > 0 && is_symbol(&data[row][col - 1]) {
            return Some((row, col - 1));
        }
        if col < col_max && is_symbol(&data[row][col + 1]) {
            return Some((row, col + 1));
        }
        if row > 0 && is_symbol(&data[row - 1][col]) {
            return Some((row - 1, col));
        }
        if row < row_max && is_symbol(&data[row + 1][col]) {
            return Some((row + 1, col));
        }
        if row < row_max && col < col_max && is_symbol(&data[row + 1][col + 1]) {
            return Some((row + 1, col + 1));
        }
        if row > 0 && col < col_max && is_symbol(&data[row - 1][col + 1]) {
            return Some((row - 1, col + 1));
        }
        if row < row_max && col > 0 && is_symbol(&data[row + 1][col - 1]) {
            return Some((row + 1, col - 1));
        }
    }

    None
}

fn is_symbol(c: &char) -> bool {
    c == &'*'
} 
