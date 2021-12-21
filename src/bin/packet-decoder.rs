fn main() {
    let input = read_input();
    let mut reader = Reader::new(&input.as_slice());
    if let Some(packet) = Packet::read(&mut reader) {
        println!("version sum: {}", version_sum(&packet));
        println!("value: {}", packet.value());
    }
}

fn read_input() -> Vec<bool> {
    let mut result = Vec::new();
    if let Ok(string) = std::fs::read_to_string("input.txt") {
        for c in string.chars() {
            let number = {
                if c >= '0' && c <= '9' {
                    Some(c as usize - '0' as usize)
                } else if c >= 'A' && c <= 'F' {
                    Some(c as usize - 'A' as usize + 10)
                } else {
                    None
                }
            };
            if let Some(number) = number {
                push_four_bit_binary_number(&mut result, number);
            }
        }
    }
    return result;
}

fn push_four_bit_binary_number(vector: &mut Vec<bool>, number: usize) {
    let mut mask = 0b1000;
    while mask != 0 {
        vector.push((mask & number) != 0);
        mask >>= 1;
    }
}

fn boolean_slice_to_number(slice: &[bool]) -> usize {
    let mut result = 0;
    for value in slice.iter() {
        result <<= 1;
        if *value {
            result |= 1;
        }
    }
    return result;
}

fn parse_literal(reader: &mut Reader) -> Option<usize> {
    let mut result = 0;
    while let Some(set_bits) = reader.read(5) {
        result <<= 4;
        result |= boolean_slice_to_number(&set_bits[1..]);
        if !set_bits[0] {
            return Some(result);
        }
    }
    return None;
}

fn version_sum(packet: &Packet) -> usize {
    let mut sum = packet.version;
    if let PacketData::Operator(_, sub_packets) = &packet.data {
        for sub_packet in sub_packets {
            sum += version_sum(sub_packet)
        }
    }
    return sum;
}
struct Reader<'s> {
    data: &'s [bool],
}

impl<'s> Reader<'s> {
    fn new(data: &[bool]) -> Reader {
        return Reader { data };
    }

    fn read(&mut self, size: usize) -> Option<&[bool]> {
        if size <= self.data.len() {
            let result = &self.data[0..size];
            self.data = &self.data[size..];
            return Some(result);
        }
        return None;
    }

    fn len(&self) -> usize {
        return self.data.len();
    }
}

struct Packet {
    version: usize,
    data: PacketData,
}

impl Packet {
    fn read(reader: &mut Reader) -> Option<Packet> {
        let version = boolean_slice_to_number(reader.read(3)?);
        let id = boolean_slice_to_number(reader.read(3)?);
        if id == 4 {
            let literal = parse_literal(reader)?;
            return Some(Packet {
                version,
                data: PacketData::Literal(literal),
            });
        } else {
            let length_mode = reader.read(1)?.get(0)?;
            let mut packet_vector = Vec::new();
            if *length_mode {
                let count = boolean_slice_to_number(reader.read(11)?);
                for _ in 0..count {
                    let sub_packet = Packet::read(reader)?;
                    packet_vector.push(sub_packet);
                }
            } else {
                let length = boolean_slice_to_number(reader.read(15)?);
                let mut sub_reader = Reader::new(reader.read(length)?);
                while sub_reader.len() > 0 {
                    let sub_packet = Packet::read(&mut sub_reader)?;
                    packet_vector.push(sub_packet);
                }
            }
            return Some(Packet {
                version,
                data: PacketData::Operator(id, packet_vector),
            });
        }
    }

    fn value(&self) -> usize {
        match &self.data {
            PacketData::Literal(value) => return *value,
            PacketData::Operator(id, sub_packets) => match id {
                0 => {
                    let mut sum = 0;
                    for packet in sub_packets {
                        sum += packet.value();
                    }
                    return sum;
                }
                1 => {
                    let mut product = 1;
                    for packet in sub_packets {
                        product *= packet.value();
                    }
                    return product;
                }
                2 => {
                    if let Some(first) = sub_packets.first() {
                        let mut min = first.value();
                        for packet in &sub_packets[1..] {
                            min = min.min(packet.value());
                        }
                        return min;
                    }
                }
                3 => {
                    if let Some(first) = sub_packets.first() {
                        let mut max = first.value();
                        for packet in &sub_packets[1..] {
                            max = max.max(packet.value());
                        }
                        return max;
                    }
                }
                5 => {
                    let mut iter = sub_packets.iter();
                    if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
                        if first.value() > second.value() {
                            return 1;
                        }
                    }
                }
                6 => {
                    let mut iter = sub_packets.iter();
                    if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
                        if first.value() < second.value() {
                            return 1;
                        }
                    }
                }
                7 => {
                    let mut iter = sub_packets.iter();
                    if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
                        if first.value() == second.value() {
                            return 1;
                        }
                    }
                }
                _ => (),
            },
        }
        return 0;
    }
}

enum PacketData {
    Literal(usize),
    Operator(usize, Vec<Packet>),
}
