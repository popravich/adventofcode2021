use std::iter;
use std::cmp;
use std::collections::HashSet;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let data = parse_input(input);

    let mut min_pos = usize::MAX;
    let mut min_fuel = usize::MAX;
    let mut min_pos2 = usize::MAX;
    let mut min_fuel2 = usize::MAX;
    let unique: HashSet<_> = data.iter().cloned().collect();

    let abs_diff = |(a, b): (&usize, usize)| cmp::max(*a, b) - cmp::min(*a, b);

    for p in unique.iter() {
        let fuel: usize = data.iter()
            .zip(iter::repeat(*p))
            .map(abs_diff)
            .sum();
        if let cmp::Ordering::Less = fuel.cmp(&min_fuel) {
            min_pos = *p;
            min_fuel = fuel;
        }
    }
    for p in 0..*unique.iter().max().expect("empty data") {
        let crabs_fuel: usize = data.iter()
            .zip(iter::repeat(p))
            .map(abs_diff)
            .map(|f| f * (1 + f) / 2)
            .sum();
        if let cmp::Ordering::Less = crabs_fuel.cmp(&min_fuel2) {
            min_pos2 = p;
            min_fuel2 = crabs_fuel;
        }
    }
    println!("Min fuel: {}", min_fuel);
    println!("Optimal position: {}", min_pos);

    println!("Min fuel (crabs version): {}", min_fuel2);
    println!("Optimal position: {}", min_pos2);

    Ok((min_fuel, min_fuel2))
}

pub fn parse_input(s: &str) -> Vec<usize> {
    s.split(',')
        .map(|x| x.trim().parse().expect("not a number"))
        .collect()
}
