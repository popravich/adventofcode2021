use wasm_bindgen::prelude::*;

pub mod shared;

pub use shared::{is_increasing, parse_measurements};


#[wasm_bindgen]
pub struct Answer {
    pub result1: usize,
    pub result2: usize,
}

#[wasm_bindgen]
pub fn get_answer(input: &str) -> Answer {
    let measures = parse_measurements(input);
    let result1 = measures
        .windows(2)
        .filter(|x| is_increasing(x))
        .count();
    let result2 = measures
        .windows(3)
        .map(|slice| slice.iter().sum::<isize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| is_increasing(x))
        .count();
    Answer {result1, result2}
}
