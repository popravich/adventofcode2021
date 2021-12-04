use std::cmp::Ordering;

pub fn main(input: &str) -> Metrics {

    let (bits_count, numbers) = parse_text(input);

    let common_bits_mask: usize = (0..bits_count)
        .into_iter()
        .rev()
        .map(|bit| numbers.mcb(bit as u32).expect("0 and 1 equally common") << bit)
        .sum();
    let gamma_rate: usize = common_bits_mask;
    let epsilon_rate = !gamma_rate & ((1<<bits_count) - 1);

    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);

    let max_bit = bits_count - 1;
    let mask = (common_bits_mask >> max_bit) & 1;
    let (mut oxygen_generator, mut co2_rating): (Vec<_>, Vec<_>) = numbers
        .into_iter()
        .partition(|n| (n >> max_bit & 1) == mask);

    for b in (0..max_bit).into_iter().rev() {
        if oxygen_generator.len() > 1 {
            let mcb = oxygen_generator.mcb_with_equal(b as u32, 1) & 1;
            oxygen_generator.retain(|n| ((n>>b) & 1) == mcb);
        }
        if co2_rating.len() > 1 {
            let mcb = co2_rating.lcb_with_equal(b as u32, 0) & 1;
            co2_rating.retain(|n| ((n>>b) & 1) == mcb);
        }
    }

    let oxygen_generator = *oxygen_generator.first().expect("could not find");
    let co2_rating = *co2_rating.first().expect("could not find");
    println!("Oxygen generator: {}", oxygen_generator);
    println!("CO2 rating: {}", co2_rating);
    println!("Life support rating: {}", oxygen_generator * co2_rating);
    Metrics {
        gamma_rate,
        epsilon_rate,
        oxygen_generator,
        co2_rating,
    }
}

#[derive(Debug)]
pub struct Metrics{
    pub gamma_rate: usize,
    pub epsilon_rate: usize,
    pub oxygen_generator: usize,
    pub co2_rating: usize,
}

impl Metrics {
    pub fn power_consumtion(&self) -> usize {
        self.gamma_rate * self.epsilon_rate
    }

    pub fn life_support_rating(&self) -> usize {
        self.oxygen_generator * self.co2_rating
    }
}

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
