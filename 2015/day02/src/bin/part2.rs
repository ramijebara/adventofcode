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

    let mut total_ribbon_length = 0;

    for data_line in data_lines {
        if let Ok(line) = data_line {
            info!("{}", line);
            total_ribbon_length += calculate_needed_ribbon(line);
        }
    }

    println!("Result: {total_ribbon_length}");
}

fn calculate_needed_ribbon(dimensions: String) -> usize {
    let mut dimension_data: Vec<usize> = dimensions
        .split('x')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    dimension_data.sort();
    info!("Sorted: {:?}", dimension_data);

    let ribbon = 2 * (dimension_data[0] + dimension_data[1]);
    let bow = dimension_data.iter().fold(1, |acc, x| acc * x);

    ribbon + bow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let l = calculate_needed_ribbon("2x3x4".to_string());
        assert_eq!(l, 34);
    }

    #[test]
    fn test_example_2() {
        let l = calculate_needed_ribbon("1x1x10".to_string());
        assert_eq!(l, 14);
    }
}
