use crate::day9::Heightmap;
use crate::day9::Point;


pub fn main(input: &str) -> Result<(usize, usize), String> {
    let mut energy_levels: Heightmap = input
        .parse().map_err(|e| format!("{}", e))?;

    // let mut to_check = HashSet::new();
    let mut flashes = Vec::new();
    let mut part1: usize = 0;
    for _ in 0..100 {
        flashes.clear();
        for p in (0..energy_levels.len()).map(Point) {
            if energy_levels.incr(&p) {
                flashes.push(p);
            }
        }
        part1 += flashes.len();
        while !flashes.is_empty() {
            let x = flashes.remove(0);
            for p in x.iter_neighbours(10, 100) {
                if energy_levels.get(&p) > 9 {
                    continue
                }
                if energy_levels.incr(&p) {
                    flashes.push(p);
                    part1 += 1;
                }
            }
        }
        energy_levels.reset();
    }

    Ok((part1, 0))
}

// Extend day9 code

impl Heightmap {
    fn incr(&mut self, p: &Point) -> bool {
        self.data[p.0] += 1;
        self.data[p.0] > 9
    }

    fn reset(&mut self) {
        for x in self.data.iter_mut() {
            if *x > 9 {
                *x = 0;
            }
        }
    }
}

impl Point {

    pub fn iter_neighbours(&self, width: usize, len: usize)
        -> impl Iterator<Item=Point>
    {
        let n = !self.is_north(width, len);
        let w = !self.is_west(width, len);
        let s = !self.is_south(width, len);
        let e = !self.is_east(width, len);
        let i = self.0;
        (0..8).into_iter()
            .filter_map(move |di| {
                match di {
                    0 if n => Some(Point(i - width)),
                    1 if n && w => Some(Point(i - width + 1)),
                    2 if w => Some(Point(i + 1)),
                    3 if w && s => Some(Point(i + 1 + width)),
                    4 if s => Some(Point(i + width)),
                    5 if s && e => Some(Point(i + width - 1)),
                    6 if e => Some(Point(i - 1)),
                    7 if e && n => Some(Point(i - 1 - width)),
                    _ => None,
                }
            })
    }
}

#[cfg(test)]
mod test {
    use crate::day11::main;

    static DATA: &str = concat!(
        "5483143223\n",
        "2745854711\n",
        "5264556173\n",
        "6141336146\n",
        "6357385478\n",
        "4167524645\n",
        "2176841721\n",
        "6882881134\n",
        "4846848554\n",
        "5283751526\n",
    );

    #[test]
    fn solution() {
        let (p1, p2) = main(DATA).expect("invalid data");
        assert_eq!(p1, 1656);
        assert_eq!(p2, 195);
    }
}
