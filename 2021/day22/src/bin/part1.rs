use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        return;
    };

    let mut operations: Vec<(String, Vec<Vec<isize>>)> = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            let operation_details: Vec<&str> = x.split(" ").collect();

            let mut entry: (String, Vec<Vec<isize>>) =
                (operation_details[0].to_string(), Vec::new());

            let details: Vec<&str> = operation_details[1].split(",").collect();

            // parse the details into x, y and z ranges
            details.iter().for_each(|&r| {
                let unparsed_range: Vec<&str> = r.split("=").collect();

                let parsed_range: Vec<isize> = unparsed_range[1]
                    .split("..")
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect();

                let range: Vec<isize> = (parsed_range[0]..parsed_range[1] + 1).collect();
                entry.1.push(range);
            });

            operations.push(entry);
        }
    }

    let mut on_cubes: HashSet<(isize, isize, isize)> = HashSet::new();

    operations.iter().for_each(|(operation, op_data)| {
        println!("found an _{}_ operation", operation);
        println!(
            "x length: {}, y length: {}, z length: {}",
            op_data[0].len(),
            op_data[1].len(),
            op_data[2].len()
        );

        op_data[0]
            .iter()
            .filter(|&x| x >= &-50 && x <= &50)
            .for_each(|x| {
                op_data[1]
                    .iter()
                    .filter(|&y| y >= &-50 && y <= &50)
                    .for_each(|y| {
                        op_data[2]
                            .iter()
                            .filter(|&z| z >= &-50 && z <= &50)
                            .for_each(|z| {
                                let cube = (*x, *y, *z);
                                if *operation == "on".to_string() {
                                    let _ = on_cubes.insert(cube);
                                } else {
                                    let _ = on_cubes.remove(&cube);
                                }
                            });
                    });
            });
    });

    println!("Total on cubes: {}", on_cubes.len());
}
