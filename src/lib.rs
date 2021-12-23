use wasm_bindgen::prelude::*;
use console_error_panic_hook;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;


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

#[wasm_bindgen]
#[derive(Debug)]
pub struct BigAnswer {
    pub result1: u64,
    pub result2: u64,
}

#[wasm_bindgen]
pub fn day6_task(input: &str) -> BigAnswer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day6::main(input).expect("invalid data");
    BigAnswer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day7_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day7::main(input).expect("invalid data");
    Answer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day8_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day8::main(input).expect("invalid data");
    Answer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day9_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day9::main(input).expect("invalid data");
    Answer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day10_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day10::main(input).expect("invalid data");
    Answer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day11_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day11::main(input).expect("invalid data");
    Answer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day12_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day12::main(input).expect("invalid data");
    Answer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day13_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day13::main(input).expect("invalid data");
    Answer {
        result1,
        result2,
    }
}

#[wasm_bindgen]
pub fn day14_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day14::main(input).expect("invalid data");
    Answer {
        result1: result1.try_into().expect("overflow"),
        result2: result2.try_into().expect("overflow"),
    }
}

#[wasm_bindgen]
pub fn day15_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day15::main(input).expect("invalid data");
    Answer {
        result1: result1.try_into().expect("overflow"),
        result2: result2.try_into().expect("overflow"),
    }
}

#[wasm_bindgen]
pub fn day16_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day16::main(input).expect("invalid data");
    Answer {
        result1: result1.try_into().expect("overflow"),
        result2: result2.try_into().expect("overflow"),
    }
}

#[wasm_bindgen]
pub fn day17_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day17::main(input).expect("invalid data");
    Answer {
        result1: result1.try_into().expect("overflow"),
        result2: result2.try_into().expect("overflow"),
    }
}

#[wasm_bindgen]
pub fn day18_task(input: &str) -> Answer {
    console_error_panic_hook::set_once();
    let (result1, result2) = day18::main(input).expect("invalid data");
    Answer {
        result1: result1.try_into().expect("overflow"),
        result2: result2.try_into().expect("overflow"),
    }
}
