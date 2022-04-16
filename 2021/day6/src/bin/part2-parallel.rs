// keeping as an example of the brute force approach. 
// it is not a practical solution because it will end 
// needing 2TB of RAM
use std::sync::Mutex;
use rayon::prelude::*;

fn main() {
    let mut data: Vec<u8> = vec![
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

    for day in 0..256 {
        let zero_counter = Mutex::new(0);

        data.par_iter_mut().for_each(|x| {
            if *x == 0 {
                *x = 6;
                let mut zero_counter = zero_counter.lock().unwrap();
                *zero_counter += 1;
            } else {
                *x -= 1;
            }
        });

        let zero_counter = zero_counter.lock().unwrap();
        for _ in 0..*zero_counter {
            data.push(8);
        }

        println!("Day: {}, Number of fish: {}", day, data.len());
    }
}
