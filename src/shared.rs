
pub fn parse_measurements(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.parse().expect("Not unsigned int"))
        .collect()
}

pub fn is_increasing(x: &[isize]) -> bool {
    matches!(x, [a, b] if a < b)
}
