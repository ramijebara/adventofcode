fn main() {
    // data
    // Player 1 starting position: 7
    // Player 2 starting position: 8

    // sample
    // Player 1 starting position: 4
    // Player 2 starting position: 8

    let mut p1_position = 7;
    let mut p2_position = 8;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut num_rolls = 0;
    let mut last_roll = 0;

    loop {
        num_rolls += 1;
        let rolls = roll_dice(last_roll);

        if num_rolls % 2 == 1 {
            print!("Player 1 ");
            print!("{rolls:?} ");
            let (new_position, new_score) = calculate_score(p1_position, p1_score, &rolls);
            p1_position = new_position;
            p1_score = new_score;
            println!("space: {p1_position}, score: {p1_score}");
        } else {
            print!("Player 2 ");
            print!("{rolls:?} ");
            let (new_position, new_score) = calculate_score(p2_position, p2_score, &rolls);
            p2_position = new_position;
            p2_score = new_score;
            println!("space: {p2_position}, score: {p2_score}");
        }

        last_roll = rolls[rolls.len() - 1];
        let dice_rolls = num_rolls * 3;

        if p1_score >= 1000 {
            println!(
                "Calculation: {p2_score} x {dice_rolls} =  {}",
                p2_score * dice_rolls
            );
            break;
        }

        if p2_score >= 1000 {
            println!(
                "Calculation: {p1_score} x {dice_rolls} =  {}",
                p1_score * dice_rolls
            );
            break;
        }
    }
}

fn roll_dice(last_roll: i32) -> Vec<i32> {
    let mut rolls: Vec<i32> = Vec::new();

    for i in 1..4 {
        let value = last_roll + i;
        if value <= 100 {
            rolls.push(value);
        } else {
            rolls.push(value - 100);
        }
    }

    rolls
}

// takes current position and score and returns position and score
fn calculate_score(cur_position: i32, curr_score: i32, rolls: &Vec<i32>) -> (i32, i32) {
    let rolls_sum: i32 = rolls.iter().sum();

    let new_position = cur_position + (rolls_sum % 10);

    if new_position <= 10 {
        (new_position, curr_score + new_position)
    } else {
        (new_position - 10, curr_score + new_position - 10)
    }
}
