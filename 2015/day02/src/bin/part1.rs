use log::{error, info};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut total_area = 0;

    for data_line in data_lines {
        if let Ok(line) = data_line {
            info!("{}", line);
            total_area += calculate_surface_area(line);
        }
    }

    println!("Result: {total_area}");
}

fn calculate_surface_area(dimensions: String) -> usize {
    // formula: 2*l*w + 2*w*h + 2*h*l + the area of the smallest side
    let dimension_data: Vec<usize> = dimensions
        .split('x')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut areas: Vec<usize> = Vec::new();

    if dimension_data.len() == 3 {
        areas.push(2 * dimension_data[0] * dimension_data[1]);
        areas.push(2 * dimension_data[1] * dimension_data[2]);
        areas.push(2 * dimension_data[0] * dimension_data[2]);

        let slack = areas.iter().min().unwrap();
        areas.push(slack / 2);
    }

    areas.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(calculate_surface_area("2x3x4".to_string()), 58);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(calculate_surface_area("1x1x10".to_string()), 43);
    }
}
