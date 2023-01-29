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
            let count = count_floors(x);
            println!("Result: {}", count);
        }
    }
}

fn count_floors(floor_instructions: String) -> isize {

    let mut count = 0;

    floor_instructions.chars().into_iter().for_each(|c| {
        match c {
            '(' => { count += 1 }
            ')' => { count -= 1 }
            _ => { error!("Unexpected instructions")}
        }
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let ex1 = "(())".to_string();
        let ex2 = "(())".to_string();
        assert_eq!(count_floors(ex1), 0);
        assert_eq!(count_floors(ex2), 0);
    }

    #[test]
    fn test_example_2() {
        let ex1 = "(((".to_string();
        let ex2 = "(()(()(".to_string();
        let ex3 = "))(((((".to_string();
        assert_eq!(count_floors(ex1), 3);
        assert_eq!(count_floors(ex2), 3);
        assert_eq!(count_floors(ex3), 3);
    }

    #[test]
    fn test_example_3() {
        let ex1 = "())".to_string();
        let ex2 = "))(".to_string();
        assert_eq!(count_floors(ex1), -1);
        assert_eq!(count_floors(ex2), -1);
    }

    #[test]
    fn test_example_4() {
        let ex1 = ")))".to_string();
        let ex2 = ")())())".to_string();
        assert_eq!(count_floors(ex1), -3);
        assert_eq!(count_floors(ex2), -3);
    }
}
