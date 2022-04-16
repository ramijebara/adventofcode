use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const RADIX: u32 = 16; // number base (base 10 in this case)

    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data: Vec<u8> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let y = &*x;
                data = y
                    .chars()
                    .map(|c| c.to_digit(RADIX).unwrap() as u8)
                    .collect::<Vec<u8>>();
            }
        }

        let buffer: String = data
            .iter()
            .fold(String::new(), |acc, c| format!("{}{:04b}", acc, c));

        let (packet, _) = parse(buffer);
        println!("{:#?}", packet);

        let res = evaluate(packet);
        println!("result: {}", res);
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


fn evaluate(packet :Packet) -> usize {
    match packet.operation {
        0 => { 
            let mut sum = 0;
            for p in packet.sub_packets {
                sum += evaluate(p);
            }
            return sum;
        }
        1 => {
            let mut product = 1;
            for p in packet.sub_packets {
                product *= evaluate(p)
            }
            return product;
        }
        2 => {
            return packet.sub_packets.iter().map(|p| evaluate(p.clone())).min().unwrap();
        }
        3 => {
            return packet.sub_packets.iter().map(|p| evaluate(p.clone())).max().unwrap();
        }
        4 => { return packet.literal_value; }
        5 => {
            if evaluate(packet.sub_packets[0].clone()) > evaluate(packet.sub_packets[1].clone()) {
                return 1;
            } else {
                return  0;
            }
        }
        6 => {
            if evaluate(packet.sub_packets[0].clone()) < evaluate(packet.sub_packets[1].clone()) {
                return 1;
            } else {
                return  0;
            }
        }
        7 => {
            if evaluate(packet.sub_packets[0].clone()) == evaluate(packet.sub_packets[1].clone()) {
                return 1;
            } else {
                return  0;
            }
        }
        _ => { return 0}
    }
}

fn parse(buffer: String) -> (Packet, String) {
    let (mut p, mut b) = get_packet(buffer).unwrap();

    if p.operation != 4 {
        if p.sub_packet_type == 0 {
            let mut spb = b[..p.literal_value].to_string();

            while spb.len() > 6 {
                let (sp, r) = parse(spb);
                p.sub_packets.push(sp);
                spb = r;
            }

            b = b[p.literal_value..].to_string();
        }
        
        if p.sub_packet_type == 1 {
            for _ in 0..p.literal_value {
                let (sp, buf) = parse(b);
                p.sub_packets.push(sp);
                b = buf;
            }
        }
    }

    (p, b)
}

fn get_packet(buffer: String) -> Result<(Packet, String), String> {
    let p_type = isize::from_str_radix(&buffer[3..6], 2).unwrap();

    if p_type == 4 {
        let l_packet = get_lit(&buffer);

        if let PacketCat::Literal(x) = l_packet.p_cat {
            Ok((Packet::new(4, 2, x), l_packet.buffer))
        } else {
            Err("Could not parse literal packet".into())
        }
    } else {
        let (op, o_packet) = get_op(&buffer);
        if let PacketCat::OperatorType0(x) = o_packet.p_cat {
            Ok((Packet::new(op, 0, x), o_packet.buffer))
        } else if let PacketCat::OperatorType1(x) = o_packet.p_cat {
            Ok((Packet::new(op, 1, x), o_packet.buffer))
        } else {
            Err("Could not parse operator packet".into())
        }
    }
}

fn get_lit(buffer: &String) -> BufferPacket {
    let buffer_len = buffer.len();
    let mut pos = 0;

    // read version
    let _version = isize::from_str_radix(&buffer[pos..pos + 3], 2).unwrap();
    pos += 3;

    // read type
    let _p_type = isize::from_str_radix(&buffer[pos..pos + 3], 2).unwrap();
    pos += 3;

    // read rest
    let mut bin_string = String::new();
    let mut keep_reading = true;
    while keep_reading == true && (buffer_len - pos) >= 5 {
        let x = &buffer[pos..(pos + 5)];
        if x.chars().nth(0).unwrap() == '0' {
            keep_reading = false;
        }
        bin_string.push_str(&x[1..]);
        pos += 5;
    }

    let literal = usize::from_str_radix(&bin_string, 2).unwrap();

    BufferPacket {
        p_cat: PacketCat::Literal(literal),
        buffer: buffer[pos..].to_string(),
    }
}

fn get_op(buffer: &String) -> (usize, BufferPacket) {
    let mut pos = 0;

    // read version
    let _version = isize::from_str_radix(&buffer[pos..pos + 3], 2).unwrap();
    pos += 3;

    // read type
    let p_type = usize::from_str_radix(&buffer[pos..pos + 3], 2).unwrap();
    pos += 3;

    let length_type_id = usize::from_str_radix(&buffer[pos..(pos + 1)], 2).unwrap();
    pos += 1;

    if length_type_id == 0 {
        let sub_packet_len = usize::from_str_radix(&buffer[pos..(pos + 15)], 2).unwrap();
        pos += 15;
        (
            p_type,
            BufferPacket {
                p_cat: PacketCat::OperatorType0(sub_packet_len),
                buffer: buffer[pos..].to_string(),
            },
        )
    } else {
        let sub_packet_len = usize::from_str_radix(&buffer[pos..(pos + 11)], 2).unwrap();
        pos += 11;
        (
            p_type,
            BufferPacket {
                p_cat: PacketCat::OperatorType1(sub_packet_len),
                buffer: buffer[pos..].to_string(),
            },
        )
    }
}

#[derive(Debug)]
struct BufferPacket {
    p_cat: PacketCat,
    buffer: String,
}

// Literal will have the literal value
// operator type 0 will have the length of the sub packet
// operator type 1 will have the number of sub packets
#[derive(Debug)]
enum PacketCat {
    Literal(usize),
    OperatorType0(usize),
    OperatorType1(usize),
}

#[derive(Debug, Clone)]
struct Packet {
    operation: usize,
    literal_value: usize,
    sub_packet_type: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn new(operation: usize, sub_packet_type: usize, literal_value: usize) -> Self {
        Packet {
            operation,
            sub_packet_type,
            literal_value,
            sub_packets: Vec::new(),
        }
    }
}
