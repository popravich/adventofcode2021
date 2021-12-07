use std::cmp;
use std::collections::HashSet;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let data = parse_input(input);

    let mut min_pos = usize::MAX;
    let mut min_fuel = usize::MAX;
    let mut min_pos2 = usize::MAX;
    let mut min_fuel2 = usize::MAX;
    let unique: HashSet<_> = data.iter().cloned().collect();

    let abs_diff = |a, b| cmp::max(a, b) - cmp::min(a, b);

    let max_pos = *unique.iter().max().expect("empty data");
    let fuels = (0..max_pos)
        .into_iter()
        .map(|p| (p, data.iter().map(|x| abs_diff(p, *x)).sum::<usize>()));
    for (pos, fuel) in fuels {
        if let cmp::Ordering::Less = fuel.cmp(&min_fuel) {
            min_pos = pos;
            min_fuel = fuel;
        }
    }

    let progression_sum = |x| x * (1 + x) / 2;
    let crab_fuels = (0..max_pos)
        .into_iter()
        .map(|p| (
            p,
            data.iter().map(|x| abs_diff(p, *x)).map(progression_sum).sum::<usize>()
        ));
    for (pos, fuel) in crab_fuels {
        if let cmp::Ordering::Less = fuel.cmp(&min_fuel2) {
            min_pos2 = pos;
            min_fuel2 = fuel;
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
