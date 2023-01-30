fn main() {
    let secret_key = "yzbqklnj".to_string();

    let number = calculate_secret_complement(&secret_key);

    println!("Result: {}", number);
}

fn calculate_secret_complement(secret_key: &String) -> usize {
    let mut secret_number = 0;

    loop {
        let secret = format!("{secret_key}{secret_number}");
        let digest = md5::compute(secret);
        let digest_string = format!("{:x}", digest);

        if digest_string.starts_with("00000") {
            break;
        }

        secret_number += 1;
    }

    secret_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = calculate_secret_complement(&"abcdef".to_string());
        assert_eq!(n, 609043);
    } 

    #[test]
    fn test_example_2() {
        let n = calculate_secret_complement(&"pqrstuv".to_string());
        assert_eq!(n, 1048970);
    } 
}
