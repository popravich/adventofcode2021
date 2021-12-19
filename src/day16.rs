
pub fn main(input: &str) -> Result<(usize, usize), String> {
    let bytes: Vec<_> = input.trim_end().as_bytes().chunks(2)
        .map(|slice| {
            let hi = (slice[0] as char).to_digit(16).expect("invalid digit") as u8;
            let lo = if slice.len() > 1 {
                (slice[1] as char).to_digit(16).expect("invalid digit") as u8
            } else {
                0
            };
            hi << 4 | lo
        })
        .collect();
    let mut bits = BitsIter::new(bytes);
    
    let mut messages = Vec::new();
    while !bits.is_empty() {
        messages.push(Message::parse(&mut bits));
    }
    let part1 = messages
        .iter()
        .map(|m| m.versions_sum() as usize)
        .sum::<usize>();

    assert_eq!(messages.len(), 1);
    let part2 = messages[0].result_value();
    Ok((part1, part2))
}

#[derive(Debug)]
pub struct Message {
    version: u32,
    packet: Packet,
}

#[derive(Debug)]
pub enum Packet {
    Literal {
        value: usize,
    },
    Sum {
        values: Vec<Message>,
    },
    Product {
        values: Vec<Message>,
    },
    Min {
        values: Vec<Message>,
    },
    Max {
        values: Vec<Message>,
    },
    Greater {
        left: Box<Message>,
        right: Box<Message>,
    },
    Less {
        left: Box<Message>,
        right: Box<Message>,
    },
    Equal {
        left: Box<Message>,
        right: Box<Message>,
    }
}

impl Message {
    pub fn parse(bits: &mut BitsIter) -> Self {
        let version = bits.take(3);
        let packet = Packet::parse(bits);
        Message { version, packet }
    }

    pub fn versions_sum(&self) -> u32 {
        use self::Packet::*;
        match self.packet {
            Sum { ref values } |
            Product { ref values } |
            Min { ref values } |
            Max { ref values } => {
                values
                    .iter()
                    .map(|m| m.versions_sum())
                    .sum::<u32>() + self.version
            }
            Greater { ref left, ref right } |
            Less { ref left, ref right } |
            Equal { ref left, ref right } => {
                left.versions_sum() + right.versions_sum()
            },
            Literal { .. } => self.version,
        }
    }

    pub fn result_value(&self) -> usize {
        self.packet.result_value()
    }
}


impl Packet {
    pub fn parse(bits: &mut BitsIter) -> Self {
        match bits.take(3) {
            4 => {
                let value = Packet::parse_literal(bits);
                Packet::Literal { value }
            }
            0 => {
                let values = match bits.take(1) {
                    0 => Packet::parse_by_bits(bits),
                    1 => Packet::parse_by_count(bits),
                    _ => unreachable!(),
                };
                Packet::Sum { values }
            }
            1 => {
                let values = match bits.take(1) {
                    0 => Packet::parse_by_bits(bits),
                    1 => Packet::parse_by_count(bits),
                    _ => unreachable!(),
                };
                Packet::Product { values }
            }
            2 => {
                let values = match bits.take(1) {
                    0 => Packet::parse_by_bits(bits),
                    1 => Packet::parse_by_count(bits),
                    _ => unreachable!(),
                };
                Packet::Min { values }
            }
            3 => {
                let values = match bits.take(1) {
                    0 => Packet::parse_by_bits(bits),
                    1 => Packet::parse_by_count(bits),
                    _ => unreachable!(),
                };
                Packet::Max { values }
            }
            5 => {
                let mut values = match bits.take(1) {
                    0 => Packet::parse_by_bits(bits),
                    1 => Packet::parse_by_count(bits),
                    _ => unreachable!(),
                };
                let left = Box::new(values.remove(0));
                let right = Box::new(values.remove(0));
                assert!(values.is_empty());
                Packet::Greater { left, right }
            }
            6 => {
                let mut values = match bits.take(1) {
                    0 => Packet::parse_by_bits(bits),
                    1 => Packet::parse_by_count(bits),
                    _ => unreachable!(),
                };
                let left = Box::new(values.remove(0));
                let right = Box::new(values.remove(0));
                assert!(values.is_empty());
                Packet::Less { left, right }
            }
            7 => {
                let mut values = match bits.take(1) {
                    0 => Packet::parse_by_bits(bits),
                    1 => Packet::parse_by_count(bits),
                    _ => unreachable!(),
                };
                let left = Box::new(values.remove(0));
                let right = Box::new(values.remove(0));
                assert!(values.is_empty());
                Packet::Equal { left, right }
            }
            _ => unreachable!(),
        }
    }

    pub fn parse_literal(bytes: &mut BitsIter) -> usize {
        let mut value = bytes.take(5) as usize;
        let mut result = value & 0b1111;
        while value & 0b10000 == 0b10000 {
            value = bytes.take(5) as usize;
            result = result << 4 | (value & 0b1111);
        }
        result
    }
    pub fn parse_by_count(bits: &mut BitsIter) -> Vec<Message> {
        let packets_len = bits.take(11);
        (0..packets_len)
            .into_iter()
            .map(|_| Message::parse(bits))
            .collect()
    }
    pub fn parse_by_bits(bits: &mut BitsIter) -> Vec<Message> {
        let mut bits_len = bits.take(15) as usize;
        bits_len = bits.bit_len() - bits_len;
        let mut value = Vec::new();
        while bits.bit_len() > bits_len {
            value.push(Message::parse(bits));
        }
        value
    }

    pub fn result_value(&self) -> usize {
        use self::Packet::*;
        match self {
            Literal { value } => *value as usize,
            Sum { values } => {
                values
                    .iter()
                    .map(|msg| msg.result_value())
                    .sum::<usize>()
            }
            Product { values } => {
                values
                    .iter()
                    .map(|msg| msg.result_value())
                    .product::<usize>()
            }
            Min { values } => {
                values
                    .iter()
                    .map(|msg| msg.result_value())
                    .min()
                    .expect("empty values")
            }
            Max { values } => {
                values
                    .iter()
                    .map(|msg| msg.result_value())
                    .max()
                    .expect("empty values")
            }
            Greater { left, right } => {
                let left = left.result_value();
                let right = right.result_value();
                if left > right {
                    1
                } else {
                    0
                }
            }
            Less { ref left, ref right } => {
                let left = left.result_value();
                let right = right.result_value();
                if left < right {
                    1
                } else {
                    0
                }
            }
            Equal { ref left, ref right } => {
                let left = left.result_value();
                let right = right.result_value();
                if left == right {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct BitsIter {
    inner: Vec<u8>,
    bits_offset: u32,
}

impl BitsIter {
    pub fn new(vec: Vec<u8>) -> Self {
        BitsIter {
            inner: vec,
            bits_offset: 0,
        }
    }
        
    pub fn take(&mut self, bits: u32) -> u32 {
        assert!(bits <= u32::BITS);
        let take = |byte: u32, shift: u32, mask_bits: u32| -> u32 {
            let mask = (1 << mask_bits) - 1;
            (byte >> shift) & mask
        };

        let off = self.bits_offset + bits;
        let shift: i32 = 8 - off as i32;
        if shift >= 0 { // from current byte
            self.bits_offset = (self.bits_offset + bits) % 8;
            let qword = if self.bits_offset == 0 {
                self.inner.remove(0).into()
            } else {
                self.inner[0].into()
            };
            take(qword, shift as u32, bits)
        } else { // need to pull more
            let bits0 = bits as i32 + shift;
            let mut result = self.take(bits0 as u32);
            // self.inner.remove(0);
            let shift = shift.abs() as u32;
            for _ in 0..shift / 8 {
                result = result << 8 | (self.inner.remove(0) as u32);
            }
            let shift = shift % 8;
            if shift > 0 {
                result = (result << shift) | self.take(shift);
            }
            result
        }
    }

    pub fn bit_len(&self) -> usize {
        self.inner.len() * 8 - self.bits_offset as usize
    }

    pub fn is_empty(&self) -> bool {
        if self.inner.is_empty() {
            return true
        }
        if self.inner.len() > 1 {
            return false
        }
        (self.inner[0] << self.bits_offset) == 0
    }
}

#[cfg(test)]
mod test {
    use crate::day16::main;
    use crate::day16::BitsIter;

    #[test]
    fn solution1() {
        let (p1, _) = main("8A004A801A8002F478")
            .expect("invalid input");
        assert_eq!(p1, 16);
    }

    #[test]
    fn solution2() {
        let (p1, _) = main("620080001611562C8802118E34")
            .expect("invalid input");
        assert_eq!(p1, 12);
    }

    #[test]
    fn solution3() {
        let (p1, _) = main("C0015000016115A2E0802F182340")
            .expect("invalid input");
        assert_eq!(p1, 23);
    }

    #[test]
    fn solution4() {
        let (p1, _) = main("A0016C880162017C3686B18A3D4780")
            .expect("invalid input");
        assert_eq!(p1, 31);
    }

    #[test]
    fn solution5() {
        let (_p1, p2) = main("C200B40A82")
            .expect("invalid input");
        assert_eq!(p2, 3);
    }

    #[test]
    fn bits_iter() {
        let mut v = BitsIter {
            inner: vec![
                0b1010_1111,
                0b0111_0000,
                0b1111_1111,
                0b0100_0000,
            ],
            bits_offset: 0,
        };
        assert_eq!(v.take(3), 0b101);
        assert_eq!(v.take(3), 0b011);
        assert_eq!(v.take(1), 0b1);
        assert_eq!(v.take(4), 0b1011);
        assert_eq!(v.take(15), 0b1000_0111_1111_101);
    }
}
