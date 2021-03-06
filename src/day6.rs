const DAYS: usize = 80;
const DAYS2: usize = 256;

const D: usize = 9;
const RESET_INDEX: usize = 6;
const NEW_FISH_INDEX: usize = 8;

pub fn main(input: &str) -> Result<(u64, u64), String> {
    let data = parse_input(input)?;

    let mut population = Population::new(&data);

    let result1 = population.advance(DAYS);
    println!("After {} days: {}", DAYS, result1);

    let result2 = Population::new(&data).advance(DAYS2);
    Ok((result1, result2))
}

pub fn parse_input(input: &str) -> Result<Vec<usize>, String> {
    let mut res = Vec::new();
    for s in input.split(',') {
        res.push(s.trim().parse().map_err(|e| format!("{}", e))?);
    }
    Ok(res)
}

#[derive(Debug)]
pub struct Population {
    days: [u64; D],
}

impl Population {

    pub fn new(state: &[usize]) -> Self {
        let mut days = [0; D];
        for d in state {
            assert!(*d <= 8);
            days[*d] += 1;
        }
        Population { days }
    }

    pub fn advance(&mut self, n: usize) -> u64 {
        for _ in 0..n {
            let reset = self.days[0];
            let newbies = self.days[0];
            // self.days.copy_within(1.., 0);
            for i in 1..D {
                self.days[i-1] = self.days[i];
            }
            self.days[RESET_INDEX] += reset;
            self.days[NEW_FISH_INDEX] = newbies;
        }
        self.size()
    }

    pub fn size(&self) -> u64 {
        self.days.iter().sum()
    }
}
