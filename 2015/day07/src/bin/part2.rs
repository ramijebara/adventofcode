use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use log::info;

use anyhow::Result;

use thiserror::Error;

const AND: &str = "AND";
const OR: &str = "OR";
const NOT: &str = "NOT";
const LSHIFT: &str = "LSHIFT";
const RSHIFT: &str = "RSHIFT";

fn main() -> Result<()> {
    env_logger::init();

    let file = File::open("src/bin/data.txt")?;

    let mut data_lines = io::BufReader::new(file).lines();

    let mut circuit: HashMap<String, String> = HashMap::new();

    while let Some(Ok(line)) = data_lines.next() {
        parse_line(line, &mut circuit);
    }

    // Overide line b value with value from part1
    // 3176 in this case
    
    *circuit.get_mut(&"b".to_string()).unwrap() = "3176".to_string();

    let result = get_signal("a".to_string(), &mut circuit)?;

    println!("Result: {:?}", result);

    Ok(())
}

fn get_signal(wire: String, circuit: &mut HashMap<String, String>) -> Result<u16> {
    let mut solved_circuit: HashMap<String, u16> = HashMap::new();

    // get knowns
    for (k, v) in circuit.iter() {
        if let Ok(num) = v.parse::<u16>() {
            solved_circuit.entry(k.trim().to_string()).or_insert(num);
        }
    }

    loop {
        if solved_circuit.contains_key(&wire) {
            info!("Length: {}", solved_circuit.len());
            return Ok(*solved_circuit.get(&wire).unwrap());
        }

        // fill blank spaces based on knowns
        for (k, v) in circuit.iter() {
            let parts = v.split(' ').collect::<Vec<_>>();

            if parts.len() == 3 {
                let left = parts[0].trim().to_string();
                let op = parts[1].trim().to_string();
                let right = parts[2].trim().to_string();

                // try left side
                if solved_circuit.contains_key(&left) {
                    let left_value = solved_circuit.get(&left).unwrap();

                    if let Ok(num) = right.parse::<u16>() {
                        solved_circuit.insert(k.to_owned(), bitwise_op(*left_value, op, num)?);
                    } else {
                        if solved_circuit.contains_key(&right) {
                            let right_value = solved_circuit.get(&right).unwrap();
                            solved_circuit
                                .insert(k.to_owned(), bitwise_op(*left_value, op, *right_value)?);
                        }
                    }
                }
                // try right side
                else if solved_circuit.contains_key(&right) {
                    let right_value = solved_circuit.get(&right).unwrap();

                    if let Ok(num) = left.parse::<u16>() {
                        solved_circuit.insert(k.to_owned(), bitwise_op(num, op, *right_value)?);
                    } else {
                        if solved_circuit.contains_key(&left) {
                            let left_value = solved_circuit.get(&left).unwrap();
                            solved_circuit
                                .insert(k.to_owned(), bitwise_op(*left_value, op, *right_value)?);
                        }
                    }
                }
            } else if parts.len() == 2 {
                let op = parts[0].trim().to_string();
                let right = parts[1].trim().to_string();
                if solved_circuit.contains_key(&parts[1].trim().to_string()) {
                    let num = solved_circuit.get(&right).unwrap();
                    solved_circuit.insert(k.to_owned(), bitwise_op(0, op, *num)?);
                }
            } else if parts.len() == 1 {
                let part = parts[0].trim().to_string();
                if solved_circuit.contains_key(&part) {
                    let num = solved_circuit.get(&part).unwrap();
                    solved_circuit.insert(k.to_owned(), *num);
                }
            }
        }
        info!("{:?}", solved_circuit);
    }
}

#[derive(Debug, Error)]
enum OpError {
    #[error("Invalid Operation")]
    InvalidOp,
}

fn bitwise_op(left: u16, op: String, right: u16) -> Result<u16> {
    match op.as_str() {
        AND => {
            return Ok(left & right);
        }
        OR => {
            return Ok(left | right);
        }
        LSHIFT => {
            return Ok(left << right);
        }
        RSHIFT => {
            return Ok(left >> right);
        }
        NOT => {
            return Ok(!right);
        }
        _ => {
            return Err(OpError::InvalidOp.into());
        }
    }
}

fn parse_line(line: String, circuit: &mut HashMap<String, String>) {
    let parts: Vec<_> = line.split(" -> ").collect();

    if parts.len() == 2 {
        circuit.insert(parts[1].to_string(), parts[0].to_string());
    }
}
