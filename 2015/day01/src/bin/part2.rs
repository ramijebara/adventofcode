use log::error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    for data_line in data_lines {
        if let Ok(x) = data_line {
            let position = find_basement(x);
            println!("Result: {}", position);
        }
    }
}

fn find_basement(floor_instructions: String) -> usize {
    let mut count = 0;

    for (i, c) in floor_instructions.chars().into_iter().enumerate() {
        match c {
            '(' => { count += 1 }
            ')' => { count -= 1 }
            _ => { error!("Unexpected instructions")}
        }

        if count == -1 {
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let ex1 = ")".to_string();
        assert_eq!(find_basement(ex1), 1);
    }

    #[test]
    fn test_example_2() {
        let ex1 = "()())".to_string();
        assert_eq!(find_basement(ex1), 5);
    }
}
