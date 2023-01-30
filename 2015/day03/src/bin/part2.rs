use log::{error, info};
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    env_logger::init();

    let mut data_lines = if let Ok(file) = File::open("src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    while let Some(Ok(line)) = data_lines.next() {
        info!("{line}");

        let number_of_houses = calculate_number_of_houses(line);

        println!("Result: {number_of_houses}");
    }
}

fn calculate_number_of_houses(directions: String) -> usize {
    let mut house_coordinates: HashSet<(isize, isize)> = HashSet::new();

    let inital_coords = (0, 0);
    let mut santa_coords = inital_coords;
    let mut robo_santa_coords = inital_coords;

    house_coordinates.insert(inital_coords);

    directions.chars().into_iter().enumerate().for_each(|(e, c)| {

        if (e % 2) == 0 {
            update_coords(&mut santa_coords, c);
            house_coordinates.insert(santa_coords.clone());
        } else {
            update_coords(&mut robo_santa_coords, c);
            house_coordinates.insert(robo_santa_coords.clone());
        }
    });

    house_coordinates.len()
}

fn update_coords(coords: &mut (isize, isize), direction: char) {
    match direction {
        '>' => {
            coords.0 += 1;
        }
        '<' => {
            coords.0 -= 1;
        }
        '^' => {
            coords.1 += 1;
        }
        'v' => {
            coords.1 -= 1;
        }
        _ => {
            error!("Invalid direction");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = calculate_number_of_houses("^v".to_string());
        assert_eq!(n, 3);
    }

    #[test]
    fn test_example_2() {
        let n = calculate_number_of_houses("^>v<".to_string());
        assert_eq!(n, 3);
    }

    #[test]
    fn test_example_3() {
        let n = calculate_number_of_houses("^v^v^v^v^v".to_string());
        assert_eq!(n, 11);
    }
}
