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
        let mut o2_data :Vec<Vec<char>> = data.clone();
        let mut co2_data :Vec<Vec<char>> = data.clone();

        //assuming that data width is constant.
        for i in 0..data_width {
            let col = o2_data.iter().map(|s| s.iter().nth(i).unwrap()).collect::<Vec<_>>();

            if col.len() == 1 {
                break;
            }

            let zero_count = col.iter().filter(|&n| *n == &'0').count();
            let one_count = col.iter().filter(|&n| *n == &'1').count();
            
            if one_count >= zero_count {
                o2_data = filter_elements(&o2_data, i, '1').clone();
            } else {
                o2_data = filter_elements(&o2_data, i, '0').clone();
            }
        }

        let o2_data_string :String = o2_data[0].clone().into_iter().collect();
        let o2_data_rate = isize::from_str_radix(&o2_data_string, 2).unwrap();

        //assuming that data width is constant.
        for i in 0..data_width {
            let col = co2_data.iter().map(|s| s.iter().nth(i).unwrap()).collect::<Vec<_>>();

            if col.len() == 1 {
                break;
            }

            let zero_count = col.iter().filter(|&n| *n == &'0').count();
            let one_count = col.iter().filter(|&n| *n == &'1').count();
            
            if zero_count <= one_count {
                co2_data = filter_elements(&co2_data, i, '0').clone();
            } else {
                co2_data = filter_elements(&co2_data, i, '1').clone();
            }
        }

        let co2_data_string :String = co2_data[0].clone().into_iter().collect();
        let co2_data_rate = isize::from_str_radix(&co2_data_string, 2).unwrap();

        println!(
            "O2 = {}, CO2 = {}, multiplied = {}", 
            o2_data_rate, 
            co2_data_rate, 
            (o2_data_rate * co2_data_rate)
        );
    } 
}

fn filter_elements(data :&Vec<Vec<char>>, col :usize, c :char) -> Vec<Vec<char>> {
    let filtered_data = data.clone().into_iter().filter(|r| {
        r[col] == c}
    ).collect();
    filtered_data
}

/// Returns an iterator to the reader of the lines of the file
/// The output is wrapped in Result for better error handling
fn read_lines<P>(filename :P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}