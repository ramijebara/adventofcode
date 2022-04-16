use std::collections::HashMap;

fn main() {
    let possible_rolls: Vec<Vec<usize>> = vec![
        vec![1, 1, 1],
        vec![1, 2, 1],
        vec![1, 3, 1],
        vec![1, 1, 2],
        vec![1, 1, 3],
        vec![1, 2, 2],
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![1, 3, 3],
        vec![2, 1, 1],
        vec![2, 2, 1],
        vec![2, 3, 1],
        vec![2, 1, 2],
        vec![2, 1, 3],
        vec![2, 2, 2],
        vec![2, 2, 3],
        vec![2, 3, 2],
        vec![2, 3, 3],
        vec![3, 1, 1],
        vec![3, 2, 1],
        vec![3, 3, 1],
        vec![3, 1, 2],
        vec![3, 1, 3],
        vec![3, 2, 2],
        vec![3, 2, 3],
        vec![3, 3, 2],
        vec![3, 3, 3],
    ];

    // sum all possibilities
    let mut rolls_sums: Vec<usize> = Vec::new();
    possible_rolls
        .iter()
        .for_each(|v| rolls_sums.push(v.iter().sum::<usize>()));
    rolls_sums.sort();

    // get unique sums
    let mut possible_sums = rolls_sums.clone();
    possible_sums.dedup();

    // create a hashmap of sum frequencies
    let mut sum_count: HashMap<usize, usize> = HashMap::new();
    possible_sums.iter().for_each(|s| {
        let count = rolls_sums.iter().filter(|&x| x == s).count();
        sum_count.insert(*s, count);
    });

    let mut game_state: Vec<GameState> = Vec::new();

    // data
    // Player 1 starting position: 7
    // Player 2 starting position: 8

    // sample
    // Player 1 starting position: 4
    // Player 2 starting position: 8

    let initial_state = GameState {
        p1_position: 7,
        p2_position: 8,
        p1_score: 0,
        p2_score: 0,
        num_of_universes: 1,
    };

    game_state.push(initial_state);

    let mut num_rolls: usize = 0;

    loop {
        num_rolls += 1;

        println!("rolling turn: {num_rolls}");

        let mut all_games_done = true;

        if num_rolls % 2 == 1 {
            let mut new_state: Vec<GameState> = Vec::new();
            game_state.iter().for_each(|state| {
                // skip winners and insert as is in the new_state vector
                if state.p1_score < 21 && state.p2_score < 21 {
                    all_games_done = false;
                    possible_sums.iter().for_each(|sum| {
                        let (new_position, new_score) =
                            calculate_score(state.p1_position, state.p1_score, *sum);

                        let multiplier = sum_count.get(&sum).unwrap();
                        let p1_state = GameState {
                            p1_position: new_position,
                            p2_position: state.p2_position,
                            p1_score: new_score,
                            p2_score: state.p2_score,
                            num_of_universes: state.num_of_universes * multiplier,
                        };
                        new_state.push(p1_state);
                    });
                } else {
                    new_state.push(state.clone());
                }
            });

            game_state = new_state;
        } else {
            // do the same for p2
            let mut new_state: Vec<GameState> = Vec::new();
            game_state.iter().for_each(|state| {
                // skip winners and insert as is in the new_state vector
                if state.p1_score < 21 && state.p2_score < 21 {
                    all_games_done = false;
                    possible_sums.iter().for_each(|sum| {
                        let (new_position, new_score) =
                            calculate_score(state.p2_position, state.p2_score, *sum);

                        let multiplier = sum_count.get(&sum).unwrap();
                        let p2_state = GameState {
                            p2_position: new_position,
                            p1_position: state.p1_position,
                            p2_score: new_score,
                            p1_score: state.p1_score,
                            num_of_universes: state.num_of_universes * multiplier,
                        };
                        new_state.push(p2_state);
                    });
                } else {
                    new_state.push(state.clone());
                }
            });

            game_state = new_state;
        }

        if all_games_done == true {
            break;
        }
    }

    let p1_universes = game_state
        .iter()
        .filter(|&s| &s.p1_score >= &21)
        .fold(0, |acc, s| acc + s.num_of_universes);

    let p2_universes = game_state
        .iter()
        .filter(|&s| &s.p2_score >= &21)
        .fold(0, |acc, s| acc + s.num_of_universes);

    println!("player 1 universes: {}", p1_universes);
    println!("player 2 universes: {}", p2_universes);
}

// takes current position, score and rolls_sum and returns position and score
fn calculate_score(cur_position: usize, curr_score: usize, rolls_sum: usize) -> (usize, usize) {
    let new_position = cur_position + (rolls_sum % 10);

    if new_position <= 10 {
        (new_position, curr_score + new_position)
    } else {
        (new_position - 10, curr_score + new_position - 10)
    }
}

#[derive(Debug, Clone)]
struct GameState {
    p1_position: usize,
    p2_position: usize,
    p1_score: usize,
    p2_score: usize,
    num_of_universes: usize,
}
