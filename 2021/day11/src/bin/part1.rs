fn main() {
    let mut data: Vec<Vec<u8>> = vec![
        vec![8, 2, 5, 8, 7, 4, 1, 2, 5, 4],
        vec![3, 3, 3, 5, 2, 8, 6, 2, 1, 1],
        vec![8, 4, 6, 8, 6, 6, 1, 3, 1, 1],
        vec![6, 1, 6, 4, 5, 7, 8, 3, 5, 3],
        vec![2, 1, 3, 8, 4, 1, 4, 5, 5, 3],
        vec![1, 7, 8, 5, 3, 8, 5, 4, 4, 7],
        vec![3, 4, 4, 1, 1, 3, 3, 7, 5, 1],
        vec![3, 5, 8, 6, 8, 6, 2, 8, 3, 7],
        vec![7, 5, 6, 8, 2, 7, 2, 8, 7, 8],
        vec![6, 8, 3, 3, 6, 4, 3, 1, 4, 4],
    ];

    let mut flashes = 0;

    for _ in 0..100 {
        let mut flashed: Vec<(usize, usize)> = Vec::new();
        for x in 0..data.len() {
            for y in 0..data[0].len() {
                flashes += calculate_step(x, y, &mut data, &mut flashed);
            }
        }
    }

    print_step(&data);

    println!("flashes: {}", flashes);
}

fn calculate_step(
    x: usize,
    y: usize,
    data: &mut Vec<Vec<u8>>,
    flashed: &mut Vec<(usize, usize)>,
) -> u32 {
    let x_len = data.len() - 1;
    let y_len = if x_len > 0 { data[0].len() - 1 } else { 0 };

    if x_len == 0 || y_len == 0 {
        return 0;
    }

    if flashed.contains(&(x, y)) {
        return 0;
    }

    if data[x][y] < 9 {
        data[x][y] += 1;
        return 0;
    }

    data[x][y] = 0;
    flashed.push((x, y));

    // corners

    if x == 0 && y == 0 {
        return 1
            + calculate_step(x + 1, y, data, flashed)
            + calculate_step(x, y + 1, data, flashed)
            + calculate_step(x + 1, y + 1, data, flashed);
    }

    if x == 0 && y == y_len {
        return 1
            + calculate_step(x + 1, y, data, flashed)
            + calculate_step(x, y - 1, data, flashed)
            + calculate_step(x + 1, y - 1, data, flashed);
    }

    if x == x_len && y == 0 {
        return 1
            + calculate_step(x - 1, y, data, flashed)
            + calculate_step(x - 1, y + 1, data, flashed)
            + calculate_step(x, y + 1, data, flashed);
    }

    if x == x_len && y == y_len {
        return 1
            + calculate_step(x - 1, y, data, flashed)
            + calculate_step(x, y - 1, data, flashed)
            + calculate_step(x - 1, y - 1, data, flashed);
    }

    // edges

    if x == 0 && (y < y_len && y > 0) {
        return 1
            + calculate_step(x, y + 1, data, flashed)
            + calculate_step(x, y - 1, data, flashed)
            + calculate_step(x + 1, y, data, flashed)
            + calculate_step(x + 1, y + 1, data, flashed)
            + calculate_step(x + 1, y - 1, data, flashed);
    }

    if x == x_len && (y < y_len && y > 0) {
        return 1
            + calculate_step(x, y + 1, data, flashed)
            + calculate_step(x, y - 1, data, flashed)
            + calculate_step(x - 1, y, data, flashed)
            + calculate_step(x - 1, y - 1, data, flashed)
            + calculate_step(x - 1, y + 1, data, flashed);
    }

    if y == 0 && (x < x_len && x > 0) {
        return 1
            + calculate_step(x + 1, y, data, flashed)
            + calculate_step(x - 1, y, data, flashed)
            + calculate_step(x, y + 1, data, flashed)
            + calculate_step(x + 1, y + 1, data, flashed)
            + calculate_step(x - 1, y + 1, data, flashed);
    }

    if y == y_len && (x < x_len && x > 0) {
        return 1
            + calculate_step(x + 1, y, data, flashed)
            + calculate_step(x - 1, y, data, flashed)
            + calculate_step(x, y - 1, data, flashed)
            + calculate_step(x + 1, y - 1, data, flashed)
            + calculate_step(x - 1, y - 1, data, flashed);
    }

    return 1
        + calculate_step(x + 1, y, data, flashed)
        + calculate_step(x - 1, y, data, flashed)
        + calculate_step(x, y + 1, data, flashed)
        + calculate_step(x, y - 1, data, flashed)
        + calculate_step(x + 1, y + 1, data, flashed)
        + calculate_step(x + 1, y - 1, data, flashed)
        + calculate_step(x - 1, y + 1, data, flashed)
        + calculate_step(x - 1, y - 1, data, flashed);
}

pub fn print_step(data: &Vec<Vec<u8>>) {
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            print!("{}", data[x][y]);
        }
        println!();
    }
}
