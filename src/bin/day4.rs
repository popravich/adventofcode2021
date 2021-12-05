use advent::day4;

static DATA: &str = include_str!("../input/day4.txt");

fn main() {

    let (score1, score2) = day4::main(DATA);
    println!("Answer1: {}", score1);
    println!("Answer2: {}", score2);
}
