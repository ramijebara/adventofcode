use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data = Vec::new();
        for line in data_lines {
            if let Ok(x) = line {
                let y = &*x;
                // clean string from obvious matches. this is recursive
                let y = clean_str(y.to_string());

                let y: Vec<char> = y.chars().collect::<Vec<_>>();
                data.push(y);
            }
        }

        let open_chars = vec!['(', '[', '{', '<'];
        let mut accumulator = 0;

        for l in 0..data.len() {
            let mut x :char = 'x';
            for c in &data[l] {
                if open_chars.contains(&c) {
                    x = *c;
                } else {
                    if match_chunk(x) != *c {
                        accumulator += check_value(*c);
                        println!("open char: {}, close char: {}, line: {}, points: {}", x, c, l, accumulator);
                        break;
                    }
                }
            }
        }
    }
}

/// Returns an iterator to the reader of the lines of the file
/// The output is wrapped in Result for better error handling
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn match_chunk(c: char) -> char {
    match c {
        '(' => return ')',
        '[' => return ']',
        '{' => return '}',
        '<' => return '>',
        _ => return 'x',
    }
}

fn check_value(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn clean_str(s: String) -> String {
    if s.contains("()") || s.contains("[]") || s.contains("{}") || s.contains("<>") {
        let y = str::replace(&*s, "()", "");
        let y = str::replace(&*y, "[]", "");
        let y = str::replace(&*y, "{}", "");
        let y = str::replace(&*y, "<>", "");
        clean_str(y)
    } else {
        return s.to_string();
    }
}
