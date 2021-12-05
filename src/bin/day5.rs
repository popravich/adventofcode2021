use advent::day5;

static DATA: &str = include_str!("../input/day5.txt");

fn main() -> Result<(), String> {
    let (result1, _result2) = day5::main(DATA)?;
    println!("Number of overlapping points: {}", result1);
    Ok(())
}
