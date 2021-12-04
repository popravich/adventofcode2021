static DATA: &str = include_str!("../input/day3.txt");

use advent::day3::{parse_text, MostCommonBit};

fn main() {

    let (bits_count, numbers) = parse_text(DATA);

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
