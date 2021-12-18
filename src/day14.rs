use std::time;
use std::collections::HashMap;


pub fn main(input: &str) -> Result<(u128, u128), String> {
    let (start, pairs) = input
        .split_once("\n\n")
        .ok_or("No separator".to_string())?;
    let map = HashMap::from_iter(pairs
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let a = chars.next().expect("no data");
            let b = chars.next().expect("no data");
            let v = chars.last().expect("no data");
            ((a, b), v)
        }));

    const PART1_STEPS: usize = 10;
    println!("Start: {}", start);

    let t0 = time::Instant::now();
    println!("building {} steps for all combinations...", PART1_STEPS);
    let mut stats = HashMap::new();
    for key in map.keys() {
        stats.insert(key, Stat::build(*key, &map, PART1_STEPS));
    }
    println!("done in {:?}", t0.elapsed());

    let mut total = Counter::default();
    for c in start.chars() {
        total.incr(c);
    }
    let chars_vec = start.chars().collect::<Vec<_>>();
    let mut part2_pairs: HashMap<(char, char), u128> = HashMap::new();
    for pair in chars_vec.windows(2).map(|s| (s[0], s[1])) {
        let s = stats.get(&pair).expect("not in map");
        total.merge(&s.chars_count);
        for (key, mult) in s.new_pairs.iter() {
            *part2_pairs.entry(*key).or_default() += mult;
        }
    }

    let max = total.max();
    let min = total.min();
    println!("Part1 result: {} - {} = {}", max, min, max - min);
    let part1 = max - min;

    println!("Starting part2");
    let t0 = time::Instant::now();
    for _ in 0..3 { // 3 more iteration by 10 steps;
        let todo = part2_pairs.drain().into_iter().collect::<Vec<_>>();
        for (key, mult1) in todo {
            let stat = stats.get(&key).expect("not in map");
            for (c, count) in stat.chars_count.counters.iter() {
                total.incr_by(*c, count * mult1);
            }
            for (key, mult2) in stat.new_pairs.iter() {
                *part2_pairs.entry(*key).or_default() += *mult2 * mult1;
            }
        }
        println!("itertation took {:?}", t0.elapsed());
    }
    println!("Done in {:?}", t0.elapsed());

    let max = total.max();
    let min = total.min();
    println!("Part2 result: {} - {} = {}", max, min, max - min);
    let part2 = max - min;

    Ok((part1, part2))
}

#[derive(Debug, Clone, Default)]
pub struct Counter {
    counters: HashMap<char, u128>,
}

impl Counter {

    pub fn incr(&mut self, c: char) {
        self.incr_by(c, 1);
    }

    pub fn incr_by(&mut self, c: char, val: u128) {
        *self.counters.entry(c).or_default() += val;
    }

    pub fn merge(&mut self, other: &Self) {
        for (k, v) in other.counters.iter() {
            *self.counters.entry(*k).or_default() += v;
        }
    }

    pub fn max(&self) -> u128 {
        *self.counters.values().max().expect("no data")
    }

    pub fn min(&self) -> u128 {
        *self.counters.values().min().expect("no data")
    }
}

#[derive(Debug)]
pub struct Stat {
    chars_count: Counter,
    new_pairs: HashMap<(char, char), u128>,
}

impl Stat {
    pub fn build(
        init: (char, char), map: &HashMap<(char, char), char>, steps: usize,
    ) -> Self {
        let mut chars_count = Counter::default();
        let mut new_pairs = HashMap::new();
        let mut data = vec![init.0, init.1];
        let mut new_chars = Vec::with_capacity(data.len() * 2);
        for _ in 0..steps {
            for key in data.windows(2).map(|s| (s[0], s[1])) {
                let next = *map.get(&key).expect("no data");
                chars_count.incr(next);
                new_chars.push(next);
            }
            for (i, c) in new_chars.drain(..).enumerate() {
                data.insert(2 * i + 1, c);
            }
        }
        for key in data.windows(2).map(|s| (s[0], s[1])) {
            *new_pairs.entry(key).or_default() += 1;
        }
        Stat {
            chars_count,
            new_pairs,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day14::main;

    static DATA: &str = concat!(
        "NNCB\n",
        "\n",
        "CH -> B\n",
        "HH -> N\n",
        "CB -> H\n",
        "NH -> C\n",
        "HB -> C\n",
        "HC -> B\n",
        "HN -> C\n",
        "NN -> C\n",
        "BH -> H\n",
        "NC -> B\n",
        "NB -> B\n",
        "BN -> B\n",
        "BB -> N\n",
        "BC -> B\n",
        "CC -> N\n",
        "CN -> C",
    );

    #[test]
    fn solution() {
        let (p1, p2) = main(DATA).expect("invalid input");
        assert_eq!(p1, 1588);
        assert_eq!(p2, 2188189693529);
    }
}
