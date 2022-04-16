// Solution does not yield an accurate result

fn main() {
    println!("Calculate population number with formula");
    // f(x) = a(1+r)^x where a = initial amount, r = growth rate, x = number of periods

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

    // new population growth estimate

    let period_80_data = 393019 as f64;

    //let rog_80 = nth_root(80.0, period_80_data/data.len() as f64);
    let rog_80 = (period_80_data / data.len() as f64).powf(0.0125);

    println!("rate of growth: {}", rog_80);

    let period_256_data = data.len() as f64 * rog_80.powi(256);

    println!(
        "Population after 256 days: {}",
        period_256_data.round() as u64
    );
    println!(
        "Population after 80 days: {}",
        (data.len() as f64 * rog_80.powi(80)).round() as u64
    );
}

// fn nth_root(n: f64, a: f64) -> f64 {
//     let      p  =  1e-9_f64 ;
//     let mut x0  =     a / n ;
//     loop {
//        let x1 = ( (n-1.0) * x0 + a / f64::powf(x0, n-1.0) ) / n;
//        if (x1-x0).abs() < (x0*p).abs() { return x1 };
//        x0 = x1
//     }
//  }
