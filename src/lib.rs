use wasm_bindgen::prelude::*;

pub mod shared;

pub mod day2;

pub use shared::{is_increasing, parse_measurements};

use day2::{Position, MoveCommand, Aim};

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
