use std::cmp::Ordering;

pub fn parse_text(input: &str) -> (usize, Vec<usize>) {
    let bits_size = input
        .lines()
        .map(|s| s.len())
        .max()
        .expect("No data lines");

    let numbers = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).expect("invalid representation"))
        .collect();
    (bits_size, numbers)
}

/// Get most/least common bit at `bit` position.
///
/// Returns `None` if 1 and 0 are equally common.
pub trait MostCommonBit {

    fn mcb(&self, bit: u32) -> Option<usize>;

    fn mcb_with_equal(&self, bit: u32, on_equal: usize) -> usize {
        self.mcb(bit).unwrap_or(on_equal)
    }

    fn lcb(&self, bit: u32) -> Option<usize> {
        self.mcb(bit).map(|b| !b)
    }

    fn lcb_with_equal(&self, bit: u32, on_equal: usize) -> usize {
        self.lcb(bit).unwrap_or(on_equal)
    }
}

impl MostCommonBit for Vec<usize> {

    fn mcb(&self, bit: u32) -> Option<usize> {
        assert!(bit < usize::BITS, "Bit is out of range");
        let (ones, nils) = self
            .iter()
            .map(|n| n >> bit & 1)
            .map(|b| (b, !b & 1))
            .fold((0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2));
        match ones.cmp(&nils) {
            Ordering::Less => Some(0),
            Ordering::Greater => Some(1),
            Ordering::Equal => None,
        }
    }
}
