use log::{error, info};
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/sample.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut packets: Vec<(String, String)> = Vec::new();
    let mut packet: (String, String) = (String::new(), String::new());
    let mut marker = 0;

    for data_line in data_lines {
        if let Ok(line) = data_line {
            match marker {
                0 => {
                    packet.0 = line;
                    marker += 1;
                }
                1 => {
                    packet.1 = line;
                    marker += 1;
                }
                _ => {
                    marker = 0;
                    packets.push(packet.clone());
                }
            }
        }
    }
    let _kfgd = 0;

    // push the last packet
    packets.push(packet.clone());

    //let parsed_packets: Vec<(PacketSide, PacketSide)> = Vec::new();

    // for (left, right) in packets {
    //     info!("Left: {}, Right: {}", left, right);
    //     info!("{:?}", parse_packet(left));
    // }

    let x = PacketChunk::List(vec![PacketChunk::Number(23), PacketChunk::List(vec![])]);
    let v = PacketChunk::Number(13);

    match x {
        PacketChunk::List(l) => {
            let y = insert_value(l, v);
            info!("INSERT TEST: {:?}", y);
        }
        _ => {
            error!("Error inserting value");
        }
    }
}

#[derive(Debug, Clone)]
enum PacketChunk {
    Number(usize),
    List(Vec<PacketChunk>),
}

// fn parse_packet(raw_packet: String, packet_chunk: PacketChunk) -> PacketChunk {
//     if raw_packet.len() > 0 {
//         if raw_packet.starts_with('[') {
//             info!("New Level: {}", &raw_packet[1..].to_string());

//             match packet_chunk {
//                 PacketChunk::List(l) => {
//                     let mut r = l;

//                     if r.len() == 0 {
//                         r.push(PacketChunk::List(vec![]));
//                     } else {
//                         match r[r.len() - 1] {
//                             PacketChunk::List(ref mut x) => {
//                                 x.push(PacketChunk::List(vec![]));
//                             }
//                             PacketChunk::Number(_) => {
//                                 r.push(PacketChunk::List(vec![]));
//                             }
//                         }
//                     }
//                     return parse_packet(raw_packet[1..].to_string(), PacketChunk::List(r));
//                 }
//                 PacketChunk::Number(_) => {
//                     return packet_chunk;
//                 }
//             }
//         } else if raw_packet.starts_with(',') || raw_packet.starts_with(']') {
//             return parse_packet(raw_packet[1..].to_string(), packet_chunk);
//         } else {
//             let datum = raw_packet
//                 .chars()
//                 .take_while(|c| c != &',' && c != &']')
//                 .collect::<String>();

//             let num = usize::from_str_radix(&datum, 10).unwrap();

//             info!("NUMBER: {}", num);
//         }
//     } else {
//         return packet_chunk;
//     }
// }

fn insert_value(list: Vec<PacketChunk>, chunk: PacketChunk) -> PacketChunk {
    let mut l = list;

    match l.last() {
        Some(PacketChunk::Number(_)) => {
            l.push(chunk);
        }
        Some(PacketChunk::List(x)) => {
            l.push(insert_value(x.clone(), chunk));
        }
        None => {
            l.push(chunk);
        }
    }

    PacketChunk::List(l)
}

// fn parse_packet(packet: String) {
//     if packet.len() > 0 {
//         if packet.starts_with('[') {
//             info!("New Level: {}", &packet[1..].to_string());
//             parse_packet(packet[1..].to_string());
//         } else if packet.starts_with(',') || packet.starts_with(']') {
//             parse_packet(packet[1..].to_string());
//         } else {
//             let datum = packet
//                 .chars()
//                 .take_while(|c| c != &',' && c != &']')
//                 .collect::<String>();

//             let num = usize::from_str_radix(&datum, 10).unwrap();

//             info!("NUMBER: {}", num);

//             parse_packet(packet[datum.len()..].to_string());
//         }
//     }
// }
