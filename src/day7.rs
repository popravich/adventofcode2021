use std::iter;
use std::cmp;
use std::collections::HashSet;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let data = parse_input(input);

    let mut min_pos = usize::MAX;
    let mut min_fuel = usize::MAX;
    let unique: HashSet<_> = data.iter().cloned().collect();
    for p in unique.iter() {
        let fuel: usize = data.iter()
            .zip(iter::repeat(*p))
            .map(|(a, b)| cmp::max(*a, b) - cmp::min(*a, b))
            .sum();
        if let cmp::Ordering::Less = fuel.cmp(&min_fuel) {
            min_pos = *p;
            min_fuel = fuel;
        }
    }
    println!("Min fuel: {}", min_fuel);
    println!("Optimal position: {}", min_pos);

    Ok((min_fuel, 0))
}

pub fn parse_input(s: &str) -> Vec<usize> {
    s.split(',')
        .map(|x| x.trim().parse().expect("not a number"))
        .collect()
}
