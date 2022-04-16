use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        return;
    };

    let mut algo: Vec<char> = Vec::new();
    let mut image: Vec<Vec<char>> = Vec::new();
    let mut i: usize = 0;

    for line in data_lines {
        if let Ok(x) = line {
            if i == 0 {
                algo = x.chars().collect();
            } else {
                // start parsing image if i > 0
                let image_line: Vec<char> = x.chars().collect();
                if image_line.len() > 0 {
                    image.push(image_line);
                }
            }

            i += 1;
        }
    }

    let mut enhanced = expand_image('.', &image);

    for i in 0..50 {
        println!("Iteration: {i}");
        // debug line
        //print_image(&enhanced);
        enhanced = enhance_image(&algo, &enhanced);

        let filler = enhanced[0][0];
        enhanced = expand_image(filler, &enhanced);
    }

    println!("number of white pixels: {}", count_white_pixels(&enhanced));
}

fn print_image(input_image: &Vec<Vec<char>>) {
    input_image.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{c}"));
        println!();
    });

    println!();
}

fn expand_image(filler: char, input_image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output_image: Vec<Vec<char>> = Vec::new();

    let rows = input_image.len();
    let columns = input_image[0].len();

    // add 10 pixcels from each side
    for _ in 0..(rows + 20) {
        let empty_line: Vec<char> = vec![filler; columns + 20];
        output_image.push(empty_line);
    }

    for r in 10..(rows + 10) {
        for c in 10..(columns + 10) {
            output_image[r][c] = input_image[r - 10][c - 10];
        }
    }

    output_image
}

fn enhance_image(algo: &Vec<char>, input_image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output_image: Vec<Vec<char>> = Vec::new();

    let rows = input_image.len();
    let columns = input_image[0].len();

    // initialize image
    for _ in 0..rows {
        let empty_line: Vec<char> = vec!['.'; columns];
        output_image.push(empty_line);
    }

    // enhance expanded region except for edges
    for r in 1..(rows - 1) {
        for c in 1..(columns - 1) {
            let pixels: Vec<char> = vec![
                input_image[r - 1][c - 1],
                input_image[r - 1][c],
                input_image[r - 1][c + 1],
                input_image[r][c - 1],
                input_image[r][c],
                input_image[r][c + 1],
                input_image[r + 1][c - 1],
                input_image[r + 1][c],
                input_image[r + 1][c + 1],
            ];

            let pixel_string: String = String::from_iter(pixels)
                .replace(".", "0")
                .replace("#", "1");

            let index: usize = usize::from_str_radix(&pixel_string, 2).unwrap();

            output_image[r][c] = algo[index];
        }
    }

    // enhance edges
    for r in 0..rows {
        for c in 0..columns {
            if r == 0 || r == (rows - 1) {
                output_image[r][c] = output_image[1][1];
            }
            if c == 0 || c == (columns - 1) {
                output_image[r][c] = output_image[1][1];
            }
        }
    }

    output_image
}

fn count_white_pixels(input_image: &Vec<Vec<char>>) -> usize {
    let mut counter: usize = 0;

    let rows = input_image.len();
    let columns = input_image[0].len();

    for r in 0..rows {
        for c in 0..columns {
            if input_image[r][c] == '#' {
                counter += 1;
            }
        }
    }

    counter
}
