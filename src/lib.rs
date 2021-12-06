use wasm_bindgen::prelude::*;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;


#[wasm_bindgen]
pub struct Answer {
    pub result1: usize,
    pub result2: usize,
}

#[wasm_bindgen]
pub fn day1_task(input: &str) -> Answer {
    let (result1, result2) = day1::main(input);
    Answer { result1, result2 }
}


#[wasm_bindgen]
pub fn day2_task(input: &str) -> Answer {
    let (pos, aimed_pos) = day2::main(input);
    Answer {
        result1: pos.product() as usize,
        result2: aimed_pos.position.product() as usize,
    }
}


#[wasm_bindgen]
pub fn day3_task(input: &str) -> Answer {
    let metrics = day3::main(input);
    Answer {
        result1: metrics.power_consumtion(),
        result2: metrics.life_support_rating(),
    }
}

#[wasm_bindgen]
pub fn day4_task(input: &str) -> Answer {
    let (result1, result2) = day4::main(input);
    Answer { result1, result2 }
}

#[wasm_bindgen]
pub fn day5_task(input: &str) -> Answer {
    let (result1, result2) = day5::main(input).expect("invalid data");
    Answer { result1, result2 }
}
