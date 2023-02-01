use log::{error, info};
use std::{
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

    let mut nice_counter = 0;
    while let Some(Ok(line)) = data_lines.next() {
        info!(
            "String: {line}, pairs: {}, sandwish: {}",
            contains_pair_that_repeats_twice(&line),
            contains_letter_sandwish(&line)
        );
        if is_nice(&line) {
            nice_counter += 1;
        }
    }

    println!("Result: {nice_counter}");
}

fn contains_pair_that_repeats_twice(line: &String) -> bool {
    let line_vector: Vec<char> = line.chars().collect();

    if line_vector.len() < 2 {
        return false;
    }

    for i in 0..(line_vector.len() - 1) {
        let p = format!("{}{}", line_vector[i], line_vector[i + 1]);
        let m: Vec<_> = line.match_indices(&p).collect();
        info!("pattern: {}, matches: {:?}", p, m);
        if m.len() > 1 {
            return true;
        }
    }
    false
}

fn contains_letter_sandwish(line: &String) -> bool {
    let line_vector: Vec<char> = line.chars().collect();

    if line_vector.len() < 3 {
        return false;
    }

    for i in 0..(line_vector.len() - 2) {
        if line_vector[i] == line_vector[i + 2] {
            return true;
        }
    }

    false
}

fn is_nice(line: &String) -> bool {
    contains_pair_that_repeats_twice(line) && contains_letter_sandwish(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let x = is_nice(&"qjhvhtzxzqqjkmpb".to_string());
        assert_eq!(x, true);
    }

    #[test]
    fn test_example_2() {
        let x = is_nice(&"xxyxx".to_string());
        assert_eq!(x, true);
    }

    #[test]
    fn test_example_3() {
        let x = is_nice(&"uurcxstgmygtbstg".to_string());
        assert_eq!(x, false);
    }

    #[test]
    fn test_example_4() {
        let x = is_nice(&"ieodomkazucvgmuy".to_string());
        assert_eq!(x, false);
    }

    #[test]
    fn test_example_5() {
        let x = is_nice(&"yyy".to_string());
        assert_eq!(x, false);
    }
}
