
pub fn main(input: &str) -> (usize, usize) {
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
    (result1, result2)
}

pub fn parse_measurements(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.parse().expect("Not unsigned int"))
        .collect()
}

pub fn is_increasing(x: &[isize]) -> bool {
    matches!(x, [a, b] if a < b)
}
