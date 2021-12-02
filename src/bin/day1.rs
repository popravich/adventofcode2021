use advent::parse_measurements;

fn main() {
    let measures = parse_measurements();
    let result = measures
        .windows(2)
        .filter(|x| is_incresing(x))
        .count();
    println!("#1; Number depth increases: {}", result);

    let result = measures
        .windows(3)
        .map(|slice| slice.iter().sum::<usize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| is_incresing(x))
        .count();
    println!("#2: Triplet sums compared: {}", result);
}

fn is_incresing(x: &[usize]) -> bool {
    matches!(x, [a, b] if a < b)
}
