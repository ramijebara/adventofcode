use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data :Vec<Vec<char>> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let x_chars :Vec<char> = x.chars().collect();
                data.push(x_chars);
            }
        }

        let data_width = data[0].len();
        let mut gamma_array :Vec<char> = Vec::new();
        let mut epsilon_array :Vec<char> = Vec::new();

        //assuming that data width is constant.
        for i in 0..data_width {
            let col = data.iter().map(|s| s.iter().nth(i).unwrap()).collect::<Vec<_>>();
            let zero_count = col.iter().filter(|&n| *n == &'0').count();
            let one_count = col.iter().filter(|&n| *n == &'1').count();

            if one_count > zero_count {
                gamma_array.push('1');
                epsilon_array.push('0');
            } else {
                gamma_array.push('0');
                epsilon_array.push('1');
            }
        }

        let gamma_string :String = gamma_array.into_iter().collect(); 
        let epsilon_string :String = epsilon_array.into_iter().collect();

        let gamma_rate = isize::from_str_radix(&gamma_string, 2).unwrap();
        let epsilon_rate = isize::from_str_radix(&epsilon_string, 2).unwrap();
        println!("gamma = {}, epsilon = {}, multiplied = {}", gamma_rate, epsilon_rate, (gamma_rate * epsilon_rate));
    }
}

/// Returns an iterator to the reader of the lines of the file
/// The output is wrapped in Result for better error handling
fn read_lines<P>(filename :P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
