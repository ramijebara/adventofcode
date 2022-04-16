use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const RADIX: u32 = 16; // number base (base 10 in this case)

    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data: Vec<u8> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let y = &*x;
                data = y
                    .chars()
                    .map(|c| c.to_digit(RADIX).unwrap() as u8)
                    .collect::<Vec<u8>>();
            }
        }

        let parsed_hex_data: String = data
            .iter()
            .fold(String::new(), |acc, c| format!("{}{:X}", acc, c));
        println!("HEX: {}", parsed_hex_data);

        let buffer: String = data
            .iter()
            .fold(String::new(), |acc, c| format!("{}{:04b}", acc, c));

        let buffer_len = buffer.len();
        println!("BIN: {}\nLEN: {}", buffer, buffer_len);

        //let mut packets :Vec<(usize, usize, String)> = Vec::new();
        let mut pos = 0;
        let mut acc = 0;

        while pos < (buffer_len - 6)  {
            // read version
            let version = isize::from_str_radix(&buffer[pos..pos + 3], 2).unwrap();

            println!("version: {}", version);
            acc += version;
            pos += 3;

            // read type
            let p_type = isize::from_str_radix(&buffer[pos..pos + 3], 2).unwrap();
            println!("Type: {}", p_type);
            pos += 3;

            // read rest
            if p_type == 4 {
                let mut keep_reading = true;
                while keep_reading == true && (buffer_len - pos) > 5 {
                    let x = &buffer[pos..(pos + 5)];
                    if x.chars().nth(0).unwrap() == '0' {
                        keep_reading = false;
                    }
                    pos += 5;
                }
            }

            if p_type != 4 {
                let length_type_id = usize::from_str_radix(&buffer[pos..(pos+1)], 2).unwrap();
                pos += 1;
                if length_type_id == 0 { pos += 15 } else { pos += 11;}
            }

            println!("debug: pos = {}, acc = {}", pos, acc);
        }
        println!("version sum: {}", acc);
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


