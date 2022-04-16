//best solution

use std::collections::HashMap;

fn main() {
    let data: Vec<u8> = vec![
        1, 4, 1, 1, 1, 1, 1, 1, 1, 4, 3, 1, 1, 3, 5, 1, 5, 3, 2, 1, 1, 2, 3, 1, 1, 5, 3, 1, 5, 1,
        1, 2, 1, 2, 1, 1, 3, 1, 5, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 4, 5, 3, 1, 1, 1, 1, 1, 1, 2, 1,
        1, 1, 1, 4, 4, 4, 1, 1, 1, 1, 5, 1, 2, 4, 1, 1, 4, 1, 2, 1, 1, 1, 2, 1, 5, 1, 1, 1, 3, 4,
        1, 1, 1, 3, 2, 1, 1, 1, 4, 1, 1, 1, 5, 1, 1, 4, 1, 1, 2, 1, 4, 1, 1, 1, 3, 1, 1, 1, 1, 1,
        3, 1, 3, 1, 1, 2, 1, 4, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 4, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 5, 1, 1, 1, 2, 2, 1, 1, 3, 5, 1, 1, 1, 1, 3, 1, 3, 3,
        1, 1, 1, 1, 3, 5, 2, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 1, 1, 1,
        1, 2, 1, 1, 1, 1, 1, 5, 1, 4, 3, 3, 1, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 3, 5, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 2, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1,
        1, 1, 1, 1, 1, 1, 1, 2, 1, 4, 4, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 2, 5, 1, 1, 4, 1, 3, 1, 1,
    ];
    //let mut data :Vec<u8> = vec![3,4,3,1,2];

    let mut tracker: HashMap<u8, usize> = HashMap::new();
    tracker.insert(0, 0);
    tracker.insert(1, 0);
    tracker.insert(2, 0);
    tracker.insert(3, 0);
    tracker.insert(4, 0);
    tracker.insert(5, 0);
    tracker.insert(6, 0);
    tracker.insert(7, 0);
    tracker.insert(8, 0);

    for i in (0..9).rev() {
        *tracker.get_mut(&i).unwrap() = data.iter().filter(|&x| *x == i).count();
    }

    for _ in 0..256 {
        let mut zero_counter = 0;

        for i in 0..9 {
            if i == 0 {
                // add the contents of 0 into counter
                zero_counter = tracker[&0];

                // empty 0
                *tracker.get_mut(&0).unwrap() = 0;
            } else {
                *tracker.get_mut(&(i - 1)).unwrap() += tracker[&i];
                *tracker.get_mut(&i).unwrap() = 0;
            }
        }

        *tracker.get_mut(&6).unwrap() += zero_counter;
        *tracker.get_mut(&8).unwrap() += zero_counter;
    }

    let mut sum = 0;

    for i in 0..9 {
        sum += tracker[&i];
        println!("tracker: {}, {}", &i, tracker[&i]);
    }

    println!("Number of fish at end of period: {}", sum);
}
