
use advent::{parse_measurements, is_increasing};

static DATA: &str = include_str!("../input/day1.txt");

fn main() {
    let measures = parse_measurements(DATA);
    let result = measures
        .windows(2)
        .filter(|x| is_increasing(x))
        .count();
    println!("#1; Number depth increases: {}", result);

    let result = measures
        .windows(3)
        .map(|slice| slice.iter().sum::<isize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| is_increasing(x))
        .count();
    println!("#2: Triplet sums compared: {}", result);
}
