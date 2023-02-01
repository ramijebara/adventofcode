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
        info!("String: {line}");
        if is_nice(&line) {
            nice_counter += 1;
        }
    }

    println!("Result: {nice_counter}");
}

fn contains_at_least_3_vowels(line: &String) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;
    for c in vowels {
        let c_matches = line.chars().filter(|x| c == *x).count();
        count += c_matches;

        if count >= 3 {
            return true;
        }
    }
    false
}

fn contains_duplicates(line: &String) -> bool {
    let l = line.to_owned();
    let mut l_chars = l.chars().peekable();

    while let Some(c) = l_chars.next() {
        if let Some(next_c) = l_chars.peek() {
            if &c == next_c {
                return true;
            }
        }
    }

    false
}

fn does_not_contain_bad_string(line: &String) -> bool {
    let banned_strings = vec!["ab", "cd", "pq", "xy"];

    for s in banned_strings {
        if line.contains(s) {
            return false;
        }
    }

    true
}

fn is_nice(line: &String) -> bool {
    contains_at_least_3_vowels(line)
        && contains_duplicates(line)
        && does_not_contain_bad_string(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let x = contains_at_least_3_vowels(&"aeiouaeiouaeiou".to_string());
        assert_eq!(x, true);
    }

    #[test]
    fn test_example_2() {
        let x = contains_duplicates(&"aaa".to_string());
        assert_eq!(x, true);
    }

    #[test]
    fn test_example_3() {
        let x = does_not_contain_bad_string(&"aaa".to_string());
        assert_eq!(x, true);
    }

    #[test]
    fn test_example_4() {
        let x = is_nice(&"ugknbfddgicrmopn".to_string());
        assert_eq!(x, true);
    }

    #[test]
    fn test_example_5() {
        let x = is_nice(&"aaa".to_string());
        assert_eq!(x, true);
    }

    #[test]
    fn test_example_6() {
        let x = is_nice(&"jchzalrnumimnmhp".to_string());
        assert_eq!(x, false);
    }

    #[test]
    fn test_example_7() {
        let x = is_nice(&"haegwjzuvuyypxyu".to_string());
        assert_eq!(x, false);
    }
    #[test]
    fn test_example_8() {
        let x = is_nice(&"dvszwmarrgswjxmb".to_string());
        assert_eq!(x, false);
    }
}
