use std::cmp::Ordering;

static DATA: &str = include_str!("../input/day3.txt");

fn main() {

    let bits_count = DATA
        .lines()
        .map(|s| s.len())
        .max()
        .expect("no data");

    let mut counts: Vec<(usize, usize)> = Vec::new();
    counts.resize_with(bits_count, Default::default);

    for line in DATA.lines() {
        for (i, c) in line
            .chars()
            .map(char_to_u8)
            .enumerate()
        {
            let v = counts[i];
            counts[i] = (v.0 + c as usize, v.1 + 1);
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (idx, (ones, zeros)) in counts
        .iter()
        .rev()
        .map(|(ones, total)| (ones, total - ones))
        .enumerate()
    {
        match ones.cmp(&zeros) {
            Ordering::Less => epsilon_rate += 1<<idx,
            Ordering::Greater => gamma_rate += 1<<idx,
            Ordering::Equal => unreachable!("ones == zeores"),
        }
    }
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
}


fn char_to_u8(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        c => unreachable!("Unexpeced char {}", c),
    }
}
