use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Answer {
    pub result1: usize,
    pub result2: usize,
}

#[wasm_bindgen]
pub fn get_answer(input: &str) -> Answer {
    let measures = parse_text(input);
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


fn parse_text(input: &str) -> Vec<isize> {
   input 
        .lines()
        .map(|l| l.parse().expect("Not unsigned int"))
        .collect()
}

fn is_increasing(x: &[isize]) -> bool {
    matches!(x, [a, b] if a < b)
}
