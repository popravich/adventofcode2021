use advent::day5;

static DATA: &str = include_str!("../input/day5.txt");

fn main() -> Result<(), String> {
    let (result1, result2) = day5::main(DATA)?;
    println!("Number of overlapping points1: {}", result1);
    println!("Number of overlapping points2: {}", result2);
    Ok(())
}
