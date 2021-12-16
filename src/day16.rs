use to_binary::BinaryString;

fn main() {
    part1("38006F45291200                ", "38006F45291200".into());
    part1("EE00D40C823060                ", "EE00D40C823060".into());
    part1(
        "8A004A801A8002F478            ",
        "8A004A801A8002F478".into(),
    );
    part1(
        "620080001611562C8802118E34    ",
        "620080001611562C8802118E34".into(),
    );
    part1(
        "C0015000016115A2E0802F182340  ",
        "C0015000016115A2E0802F182340".into(),
    );
    part1(
        "A0016C880162017C3686B18A3D4780",
        "A0016C880162017C3686B18A3D4780".into(),
    );
    part1("input                         ", read_input());
    part2("C200B40A82                ", "C200B40A82".into());
    part2("04005AC33890              ", "04005AC33890".into());
    part2("880086C3E88112            ", "880086C3E88112".into());
    part2("CE00C43D881120            ", "CE00C43D881120".into());
    part2("D8005AC2A8F0              ", "D8005AC2A8F0".into());
    part2("F600BC2D8F                ", "F600BC2D8F".into());
    part2("9C005AC2F8F0              ", "9C005AC2F8F0".into());
    part2(
        "9C0141080250320F1802104A08",
        "9C0141080250320F1802104A08".into(),
    );
    part2("input                     ", read_input());
}

fn part1(title: &str, input: String) {
    let packets = parse(input);
    let sum: usize = packets.iter().map(|p| p.version_id).sum();
    println!("Part 1 - {} -> {}", title, sum);
}

fn part2(title: &str, input: String) {
    let packets = parse(input);
    let value = evaluate(packets);
    println!("Part 2 - {} -> {}", title, value);
}

struct Packet {
    version_id: usize,
    type_id: usize,
    value: usize,
    sub_packet_ids: Vec<usize>,
}

fn parse(input: String) -> Vec<Packet> {
    let input = to_binary(input);
    let mut packets = Vec::new();
    let mut offset = 0;
    parse_packet(&input, &mut offset, &mut packets);
    return packets;
}

fn parse_packet(input: &Vec<bool>, offset: &mut usize, packets: &mut Vec<Packet>) -> usize {
    let version_id = decode(&input, *offset, 3);
    *offset += 3;
    let type_id = decode(&input, *offset, 3);
    *offset += 3;

    let packet_id = packets.len();
    packets.push(Packet {
        version_id: version_id,
        type_id: type_id,
        value: 0,
        sub_packet_ids: Vec::new(),
    });
    if type_id == 4 {
        packets[packet_id].value = parse_number(&input, offset);
        return packet_id;
    } else {
        let length_type_id = input[*offset];
        *offset += 1;

        if length_type_id {
            let n_sub_packets = decode(&input, *offset, 11);
            *offset += 11;
            for _ in 0..n_sub_packets {
                let sub_packet_id = parse_packet(input, offset, packets);
                packets[packet_id].sub_packet_ids.push(sub_packet_id);
            }
            return packet_id;
        } else {
            let n_bits = decode(&input, *offset, 15);
            *offset += 15;
            let end_offset = *offset + n_bits;
            loop {
                let sub_packet_id = parse_packet(input, offset, packets);
                packets[packet_id].sub_packet_ids.push(sub_packet_id);
                if *offset == end_offset {
                    break;
                }
            }
            return packet_id;
        }
    }
}

fn parse_number(input: &Vec<bool>, offset: &mut usize) -> usize {
    let mut number_bits: Vec<bool> = Vec::new();

    loop {
        let first_bit = input[*offset];
        for i in 1..5 {
            number_bits.push(input[*offset + i])
        }
        *offset += 5;
        if !first_bit {
            break;
        }
    }

    return decode(&number_bits, 0, number_bits.len());
}

fn evaluate(packets: Vec<Packet>) -> usize {
    return evaluate_packet(&packets, 0);
}

fn evaluate_packet(packets: &Vec<Packet>, packet_id: usize) -> usize {
    let sub_values = packets[packet_id]
        .sub_packet_ids
        .iter()
        .map(|&id| evaluate_packet(&packets, id))
        .collect::<Vec<usize>>();

    match packets[packet_id].type_id {
        0 => sub_values.iter().sum(),
        1 => sub_values.iter().product(),
        2 => *sub_values.iter().min().unwrap(),
        3 => *sub_values.iter().max().unwrap(),
        4 => packets[packet_id].value,
        5 => {
            if sub_values[0] > sub_values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if sub_values[0] < sub_values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if sub_values[0] == sub_values[1] {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn decode(input: &Vec<bool>, offset: usize, len: usize) -> usize {
    let iter = input.iter().skip(offset).take(len);

    let mut result = 0;
    for b in iter {
        result = (result << 1) + if *b { 1 } else { 0 };
    }
    return result;
}

fn to_binary(s: String) -> Vec<bool> {
    return BinaryString::from_hex(s.trim().lines().next().unwrap())
        .unwrap()
        .to_string()
        .chars()
        .map(|c| c == '1')
        .collect::<Vec<bool>>();
}

fn read_input() -> String {
    return std::fs::read_to_string("assets/day16.txt").expect("Unable to read file");
}
