use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod shared;

pub mod day2;
pub mod day3;

pub use shared::{is_increasing, parse_measurements};

use day2::{Position, MoveCommand, Aim};
use day3::MostCommonBit;

#[wasm_bindgen]
pub struct Answer {
    pub result1: isize,
    pub result2: isize,
}

#[wasm_bindgen]
pub fn get_answer(input: &str) -> Answer {
    let measures = parse_measurements(input);
    let result1 = measures
        .windows(2)
        .filter(|x| is_increasing(x))
        .count() as isize;
    let result2 = measures
        .windows(3)
        .map(|slice| slice.iter().sum::<isize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| is_increasing(x))
        .count() as isize;
    Answer {result1, result2}
}


#[wasm_bindgen]
pub fn day2_task(input: &str) -> Answer {

    let mut pos = Position::default();
    let mut aimed_pos = Aim::default();

    let commands = input
        .lines()
        .map(|line| line.parse::<MoveCommand>().expect("Invalid command"));

    for cmd in commands {
        pos.apply(&cmd);
        aimed_pos.apply(&cmd);
    }
    Answer {
        result1: pos.product(),
        result2: aimed_pos.position.product(),
    }
}


#[wasm_bindgen]
pub fn day3_task(input: &str) -> Answer {

    console::time_with_label("parse_text");
    let (bits_count, numbers) = day3::parse_text(input);
    console::time_end_with_label("parse_text");

    let common_bits_mask: usize = (0..bits_count)
        .into_iter()
        .rev()
        .map(|bit| numbers.mcb(bit as u32).expect("0 and 1 equally common") << bit)
        .sum();

    let gamma_rate: usize = common_bits_mask;
    let epsilon_rate = !gamma_rate & ((1<<bits_count) - 1);

    let result1 = (gamma_rate * epsilon_rate) as isize;

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
    let result2 = (oxygen_generator * co2_rating) as isize;
    Answer { result1, result2 }
}
