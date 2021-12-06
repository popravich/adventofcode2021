use advent::day6;

static DATA: &str = include_str!("../input/day6.txt");

fn main() -> Result<(), String> {
    println!("Day6");
    let (result1, result2) = day6::main(DATA)?;
    println!("Result1: {}", result1);
    println!("Result2: {}", result2);
    Ok(())
}
