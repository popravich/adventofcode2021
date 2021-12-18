use std::env;

use advent::{
    day1,
    day2,
    day3,
    day4,
    day5,
    day6,
    day7,
    day8,
    day9,
    day10,
    day11,
    day12,
    day13,
    day14,
};

static DAY1: &str = include_str!("./input/day1.txt");
static DAY2: &str = include_str!("./input/day2.txt");
static DAY3: &str = include_str!("./input/day3.txt");
static DAY4: &str = include_str!("./input/day4.txt");
static DAY5: &str = include_str!("./input/day5.txt");
static DAY6: &str = include_str!("./input/day6.txt");
static DAY7: &str = include_str!("./input/day7.txt");
static DAY8: &str = include_str!("./input/day8.txt");
static DAY9: &str = include_str!("./input/day9.txt");
static DAY10: &str = include_str!("./input/day10.txt");
static DAY11: &str = include_str!("./input/day11.txt");
static DAY12: &str = include_str!("./input/day12.txt");
static DAY13: &str = include_str!("./input/day13.txt");
static DAY14: &str = include_str!("./input/day14.txt");

fn main() -> Result<(), String> {
    let day = env::args()
        .nth(1)
        .ok_or("Day number is required".to_string())
        .and_then(|s| s.trim().parse().map_err(|e| format!("{}", e)))?;
    match day {
        1 => {
            let (answer1, answer2) = day1::main(DAY1);
            println!("#1; Number depth increases: {}", answer1);
            println!("#2: Triplet sums compared: {}", answer2);
        }
        2 => {
            let (pos, aimed_pos) = day2::main(DAY2);
            println!("Our position is {:?}: {}", pos, pos.product());
            println!("Okay, now our position is {:?}: {}",
                aimed_pos, aimed_pos.position.product());
        }
        3 => {
            let metrics = day3::main(DAY3);

            println!("Gamma rate: {}", metrics.gamma_rate);
            println!("Epsilon rate: {}", metrics.epsilon_rate);
            println!("Power consumption: {}", metrics.power_consumtion());

            println!("Oxygen generator: {}", metrics.oxygen_generator);
            println!("CO2 rating: {}", metrics.co2_rating);
            println!("Life support rating: {}", metrics.life_support_rating());
        }
        4 => {
            let (score1, score2) = day4::main(DAY4);
            println!("Answer1: {}", score1);
            println!("Answer2: {}", score2);
        }
        5 => {
            let (result1, result2) = day5::main(DAY5)?;
            println!("Number of overlapping points1: {}", result1);
            println!("Number of overlapping points2: {}", result2);
        }
        6 => {
            let (result1, result2) = day6::main(DAY6)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        7 => {
            let (result1, result2) = day7::main(DAY7)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        8 => {
            let (result1, result2) = day8::main(DAY8)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        9 => {
            let (result1, result2) = day9::main(DAY9)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        10 => {
            let (result1, result2) = day10::main(DAY10)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        11 => {
            let (result1, result2) = day11::main(DAY11)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        12 => {
            let (result1, result2) = day12::main(DAY12)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        13 => {
            let (result1, result2) = day13::main(DAY13)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        14 => {
            let (result1, result2) = day14::main(DAY14)?;
            println!("Result1: {}", result1);
            println!("Result2: {}", result2);
        }
        x => return Err(format!("Invaild day number: {}", x))
    }
    Ok(())
}
