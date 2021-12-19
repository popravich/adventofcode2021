
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
    println!("Vec:\n{}\n{:?}", input, bytes);
    let mut bits = BitsIter::new(bytes);
    
    let mut messages = Vec::new();
    while !bits.is_empty() {
        messages.push(Message::parse(&mut bits));
    }
    let part1 = messages
        .iter()
        .map(|m| m.versions_sum() as usize)
        .sum::<usize>();
    Ok((part1, 0))
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
    Operator {
        kind: u32,
        value: Vec<Message>
    }
}

impl Message {
    pub fn parse(bits: &mut BitsIter) -> Self {
        let version = bits.take(3);
        let packet = Packet::parse(bits);
        Message { version, packet }
    }

    pub fn versions_sum(&self) -> u32 {
        match self.packet {
            Packet::Operator { ref value, .. } => {
                value
                    .iter()
                    .map(|m| m.versions_sum())
                    .sum::<u32>() + self.version
            }
            _ => self.version,
        }
    }
}

impl Packet {
    pub fn parse(bits: &mut BitsIter) -> Self {
        match bits.take(3) {
            4 => {
                let value = Packet::parse_literal(bits);
                Packet::Literal { value }
            }
            kind => {
                let value = match bits.take(1) {
                    0 => {
                        let mut bits_len = dbg!(bits.take(15)) as usize;
                        bits_len = bits.bit_len() - bits_len;
                        let mut value = Vec::new();
                        while bits.bit_len() > bits_len {
                            value.push(Message::parse(bits));
                        }
                        value
                    }
                    1 => {
                        let packets_len = bits.take(11);
                        (0..packets_len)
                            .into_iter()
                            .map(|_| Message::parse(bits))
                            .collect()
                    }
                    _ => unreachable!(),
                };
                Packet::Operator { kind, value }
            }
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
        println!("take {}; {:X}; off:{}", bits, self.inner[0], self.bits_offset);
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
                println!("take' {}; {:X}; off:{}", 8, self.inner[0], self.bits_offset);
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
        let (p1, p2) = main("8A004A801A8002F478")
            .expect("invalid input");
        assert_eq!(p1, 16);
        assert_eq!(p2, 0);
    }

    #[test]
    fn solution2() {
        let (p1, p2) = main("620080001611562C8802118E34")
            .expect("invalid input");
        assert_eq!(p1, 12);
        assert_eq!(p2, 0);
    }

    #[test]
    fn solution3() {
        let (p1, p2) = main("C0015000016115A2E0802F182340")
            .expect("invalid input");
        assert_eq!(p1, 23);
        assert_eq!(p2, 0);
    }

    #[test]
    fn solution4() {
        let (p1, p2) = main("A0016C880162017C3686B18A3D4780")
            .expect("invalid input");
        assert_eq!(p1, 31);
        assert_eq!(p2, 0);
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
