use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
    ops::Deref,
    path::Path,
};

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data: Vec<SailfishNum> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let (sfn, _) = parse_sfn(x);
                data.push(sfn);
            }
        }

        if data.len() > 1 {
            let mut sum = data[0].clone();

            for x in 1..data.len() {
                sum = sum.add(data[x].clone());
                sum = sum.reduce();

            }

            println!("sum: {}", sum);
            println!("mag: {}", sum.magnitude());
        }
    }
}

/// Returns an iterator to the reader of the lines of the file
/// The output is wrapped in Result for better error handling
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
struct SailfishNum {
    left: Box<SailfishNumSide>,
    right: Box<SailfishNumSide>,
}

#[derive(Debug, Clone)]
enum SailfishNumSide {
    Number(usize),
    Pair(SailfishNum),
}

impl SailfishNum {
    pub fn new(left: SailfishNumSide, right: SailfishNumSide) -> Self {
        SailfishNum {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn stringify(&self) -> String {
        let mut s = "[".to_string();
        if let SailfishNumSide::Number(x) = self.left.deref() {
            s.push_str(&format!("{}", x).to_owned());
        } else if let SailfishNumSide::Pair(y) = self.left.deref() {
            s.push_str(&y.stringify().to_owned());
        }
        s.push(',');
        if let SailfishNumSide::Number(x) = self.right.deref() {
            s.push_str(&format!("{}", x).to_owned());
        } else if let SailfishNumSide::Pair(y) = self.right.deref() {
            s.push_str(&y.stringify().to_owned());
        }
        s.push(']');

        s
    }

    pub fn magnitude(&self) -> usize {
        let mut s_left = 0;
        let mut s_right = 0;

        if let SailfishNumSide::Number(x) = self.left.deref() {
            s_left += 3*x;

        } else if let SailfishNumSide::Pair(y) = self.left.deref() {
            s_left += 3*y.magnitude();
        }

        if let SailfishNumSide::Number(x) = self.right.deref() {
            s_right += 2*x;
        } else if let SailfishNumSide::Pair(y) = self.right.deref() {
            s_right += 2*y.magnitude();
        }

        s_left + s_right
    }

    pub fn add(&self, num: SailfishNum) -> Self {
        let left = SailfishNumSide::Pair(self.clone());
        let right = SailfishNumSide::Pair(num);

        SailfishNum::new(left, right)
    }

    pub fn reduce(&self) -> Self {
        let stringified = self.stringify();

        let mut flat_sfn: Vec<u8> = stringified
            .chars()
            .map(|c| {
                if let Some(n) = c.to_digit(10) {
                    return n as u8;
                }
                c as u8
            })
            .collect();

        loop {
            let l = flat_sfn.len();
            flat_sfn = explode(&mut flat_sfn);

            // skip the split if we just exploded
            if l != flat_sfn.len() {
                continue;
            }

            flat_sfn = split(&mut flat_sfn);

            if l == flat_sfn.len() {
                break;
            }
        }

        let res = flat_sfn.iter().fold(String::new(), |acc, x| match *x {
            91 => {
                format!("{}{}", acc, "[")
            }
            93 => {
                format!("{}{}", acc, "]")
            }
            44 => {
                format!("{}{}", acc, ",")
            }
            _ => {
                format!("{}{}", acc, *x)
            }
        });

        let (res_sfn, _) = parse_sfn(res);

        res_sfn
    }
}

impl fmt::Display for SailfishNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

fn not_special(x: u8) -> bool {
    let open: u8 = 91;
    let close: u8 = 93;
    let comma: u8 = 44;

    x != open && x != close && x != comma
}

pub fn explode(flat_sfn: &mut Vec<u8>) -> Vec<u8> {
    let open: u8 = 91;
    let close: u8 = 93;

    if flat_sfn.len() < 13 {
        return flat_sfn[0..flat_sfn.len()].to_vec();
    }

    let mut res: Vec<u8> = Vec::new();

    let mut level = 0;

    for i in 0..flat_sfn.len() {
        if flat_sfn[i] == open {
            level += 1;
        }
        if flat_sfn[i] == close {
            level -= 1;
        }

        // found a pair to explode
        if (level > 4) && not_special(flat_sfn[i + 1]) && not_special(flat_sfn[i + 3]) {
            let left = flat_sfn[i + 1];
            let right = flat_sfn[i + 3];

            for j in (0..(i - 1)).rev() {
                if not_special(flat_sfn[j]) {
                    flat_sfn[j] += left;
                    break;
                }
            }

            for l in (i + 5)..flat_sfn.len() {
                if not_special(flat_sfn[l]) {
                    flat_sfn[l] += right;
                    break;
                }
            }

            for k in i..(i + 5) {
                flat_sfn[k] = 0;
            }

            res.append(&mut flat_sfn[0..i].to_vec());
            res.push(0);
            res.append(&mut flat_sfn[(i + 5)..flat_sfn.len()].to_vec());

            break;
        }
    }

    if res.len() == 0 {
        return flat_sfn[0..flat_sfn.len()].to_vec();
    }
    
    res
}

pub fn split(flat_sfn: &mut Vec<u8>) -> Vec<u8> {
    let open: u8 = 91;
    let close: u8 = 93;
    let comma: u8 = 44;


    let mut res: Vec<u8> = Vec::new();

    for i in 0..flat_sfn.len() {

        // found a pair to explode
        if (flat_sfn[i] > 9) && not_special(flat_sfn[i]) {

            let n = flat_sfn[i] as f32;
            let left = (n/2.0).floor() as u8;
            let right = (n/2.0).ceil() as u8;

            res.append(&mut flat_sfn[0..i].to_vec());
            res.push(open);
            res.push(left);
            res.push(comma);
            res.push(right);
            res.push(close);
            res.append(&mut flat_sfn[(i + 1)..flat_sfn.len()].to_vec());

            break;
        }
    }

    if res.len() == 0 {
        return flat_sfn[0..flat_sfn.len()].to_vec();
    }
    
    res
}

// this parses the sailfish number into a binary tree. an overkill
// but I wanted to see how it can be done in rust.

fn parse_sfn(mut num_exp: String) -> (SailfishNum, String) {
    // if we have a number on the left

    if let Ok(x) = usize::from_str_radix(&num_exp[0..1], 10) {
        let left = SailfishNumSide::Number(x);
        num_exp = num_exp[1..].to_string();

        // skip comma and close square bracket
        if let Some(',') = num_exp.chars().peekable().next() {
            num_exp = num_exp[1..].to_string();
        }

        // if there is a next character try to parse it.
        if let Some(_) = num_exp.chars().peekable().next() {
            if let Ok(x) = usize::from_str_radix(&num_exp[0..1], 10) {
                let right = SailfishNumSide::Number(x);
                num_exp = num_exp[1..].to_string();

                // chomp ']' chars at the end of the expression
                while let Some(']') = num_exp.chars().peekable().next() {
                    num_exp = num_exp[1..].to_string();
                }

                return (SailfishNum::new(left, right), num_exp);
            }
        }

        // assume we have a pair and proceed accordingly
        let (p, s) = parse_sfn(num_exp[1..].to_string());
        return (SailfishNum::new(left, SailfishNumSide::Pair(p)), s);
    }

    // if we have a pair on the left character
    let (p, s) = parse_sfn(num_exp[1..].to_string());

    // if s is consumed return, we are done
    if s.len() == 0 {
        return (p, s);
    }

    let left = SailfishNumSide::Pair(p);
    num_exp = s;

    // skip comma and close square bracket
    if let Some(',') = num_exp.chars().peekable().next() {
        num_exp = num_exp[1..].to_string();
    }

    // if there is a next character try to parse it.
    if let Some(_) = num_exp.chars().peekable().next() {
        if let Ok(x) = usize::from_str_radix(&num_exp[0..1], 10) {
            let right = SailfishNumSide::Number(x);
            num_exp = num_exp[1..].to_string();

            // chomp ']' chars at the end of the expression
            while let Some(']') = num_exp.chars().peekable().next() {
                num_exp = num_exp[1..].to_string();
            }

            return (SailfishNum::new(left, right), num_exp);
        }
    }

    // assume we have a pair and proceed accordingly
    let (p, s) = parse_sfn(num_exp[1..].to_string());
    return (SailfishNum::new(left, SailfishNumSide::Pair(p)), s);
}
