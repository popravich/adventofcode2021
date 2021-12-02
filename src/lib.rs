static DATA: &str = include_str!("input/day1.txt");


pub fn parse_measurements() -> Vec<usize> {
    DATA
        .lines()
        .map(|l| l.parse().expect("Not unsigned int"))
        .collect()
}
