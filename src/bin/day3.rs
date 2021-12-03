use std::cmp::Ordering;

static DATA: &str = include_str!("../input/day3.txt");

fn main() {

    let bits_count = DATA
        .lines()
        .map(|s| s.len())
        .max()
        .expect("no data");

    let numbers: Vec<_> = DATA
        .lines()
        .map(|line| usize::from_str_radix(line, 2).expect("non binary number"))
        .collect();

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

    let oxygen_generator = oxygen_generator.first().expect("could not find");
    let co2_rating = co2_rating.first().expect("could not find");
    println!("Oxygen generator: {}", oxygen_generator);
    println!("CO2 rating: {}", co2_rating);
    println!("Life support rating: {}", oxygen_generator * co2_rating);
}


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
