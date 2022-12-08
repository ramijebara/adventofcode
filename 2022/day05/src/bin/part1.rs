use std::{io::{self, BufRead}, fs::File};
use log::{error, info};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    /*  Sample cargo stack
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
    */

    // let mut cargo_stack = vec!{
    //     vec![],
    //     vec!['Z', 'N'],
    //     vec!['M', 'C', 'D'],
    //     vec!['P']
    // };

    /* Puzzle cargo stack
[N]         [C]     [Z]            
[Q] [G]     [V]     [S]         [V]
[L] [C]     [M]     [T]     [W] [L]
[S] [H]     [L]     [C] [D] [H] [S]
[C] [V] [F] [D]     [D] [B] [Q] [F]
[Z] [T] [Z] [T] [C] [J] [G] [S] [Q]
[P] [P] [C] [W] [W] [F] [W] [J] [C]
[T] [L] [D] [G] [P] [P] [V] [N] [R]
 1   2   3   4   5   6   7   8   9 
    */

    let mut cargo_stack = vec!{
        vec![],
        vec!['T', 'P', 'Z', 'C', 'S', 'L', 'Q', 'N'],
        vec!['L', 'P', 'T', 'V', 'H', 'C', 'G'],
        vec!['D', 'C', 'Z', 'F'],
        vec!['G', 'W', 'T', 'D', 'L', 'M', 'V', 'C'],
        vec!['P', 'W', 'C'],
        vec!['P', 'F', 'J', 'D', 'C', 'T', 'S', 'Z'],
        vec!['V', 'W', 'G', 'B', 'D'],
        vec!['N', 'J', 'S', 'Q', 'H', 'W'],
        vec!['R', 'C', 'Q', 'F', 'S', 'L', 'V']
    };

    let mut instructions = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            let x = x.replace("move ", "");
            let x = x.replace(" from ", ",");
            let x = x.replace(" to ", ",");

            let mut instruction = Vec::new();

            // Qty, From, To
            let split_data: Vec<&str> = x.split(",").into_iter().collect();

            for datum in split_data {
                instruction.push(str::parse::<usize>(datum).unwrap());
            }

            instructions.push(instruction);
        }
    }

    info!("{:?}", instructions);

    instructions.iter().for_each(|i| {
        for _ in 0..i[0] {
            let c = cargo_stack[i[1]].pop().unwrap();
            cargo_stack[i[2]].push(c);
        }
    });

    println!("Result:");

    for i in 1..cargo_stack.len() {
        print!("{}", cargo_stack[i][cargo_stack[i].len() - 1]);
    }

    println!();
}
