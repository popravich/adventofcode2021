use wasm_bindgen::prelude::*;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;


#[wasm_bindgen]
pub struct Answer {
    pub result1: isize,
    pub result2: isize,
}

#[wasm_bindgen]
pub fn day1_task(input: &str) -> Answer {
    let (result1, result2) = day1::main(input);
    Answer {
        result1: result1 as isize,
        result2: result2 as isize,
    }
}


#[wasm_bindgen]
pub fn day2_task(input: &str) -> Answer {
    let (pos, aimed_pos) = day2::main(input);
    Answer {
        result1: pos.product(),
        result2: aimed_pos.position.product(),
    }
}


#[wasm_bindgen]
pub fn day3_task(input: &str) -> Answer {

    let metrics = day3::main(input);
    Answer {
        result1: metrics.power_consumtion() as isize,
        result2: metrics.life_support_rating() as isize,
    }
}
