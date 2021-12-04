
use advent::day1;

static DATA: &str = include_str!("../input/day1.txt");

fn main() {
    let (answer1, answer2) = day1::main(DATA);
    println!("#1; Number depth increases: {}", answer1);
    println!("#2: Triplet sums compared: {}", answer2);
}
