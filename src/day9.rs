use std::str::FromStr;
use std::collections::HashSet;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let heights: Heightmap = input.parse()?;

    let mut i = 0;
    let mut extremums = Vec::new();
    while i < heights.len() {
        let p = Point(i);
        if p.is_minimum(&heights) {
            extremums.push((p, heights.get(&p) as usize));
            i += 2;
            continue
        }
        i += 1;
    }

    let result1 = extremums
        .iter()
        .map(|&(_, h)| h)
        .sum::<usize>() + extremums.len();

    let mut top_basins = [0usize; 4];

    let mut visited = HashSet::new();
    let mut to_check = Vec::new();

    for p in extremums.iter().map(|&(i, _)| i) {
        assert!(p.is_basin(&heights));
        to_check.clear();
        to_check.push(p);
        visited.clear();
        visited.insert(p);
        while !to_check.is_empty() {
            let p = to_check.remove(0);
            to_check.extend(p.iter_adjacent(heights.width, heights.len())
                .filter(|x| !visited.contains(x))
                .filter(|x| x.is_basin(&heights)));
            visited.extend(&to_check);
        }
        top_basins[0] = visited.len();
        top_basins.sort();
    }

    let result2 = top_basins[1..].iter().fold(1, |a, b| a * b);
    Ok((result1, result2))
}


#[derive(Debug, Default)]
pub struct Heightmap {
    pub width: usize,
    pub data: Vec<u8>,
}
impl FromStr for Heightmap {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let width = val.find('\n').ok_or("no newline in data".to_string())?;
        let data = val
            .lines()
            .flat_map(|l| l.as_bytes())
            .map(|&b| {
                if b >= b'0' && b <= b'9'  {
                    b - b'0'
                } else {
                    unreachable!("unexpected digit")
                }
            })
            .collect();
        Ok(Heightmap { width, data, })
    }
}

impl Heightmap {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, p: &Point) -> u8 {
        self.data[p.0]
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Point(pub usize);

impl Point {
    pub fn is_north(&self, width: usize, _len: usize) -> bool {
        self.0 < width
    }
    pub fn is_south(&self, width: usize, len: usize) -> bool {
        self.0 >= len - width
    }
    pub fn is_east(&self, width: usize, _len: usize) -> bool {
        self.0 % width == 0
    }
    pub fn is_west(&self, width: usize, _len: usize) -> bool {
        self.0 % width == width - 1
    }

    pub fn is_minimum(&self, hm: &Heightmap) -> bool {
        let h = hm.get(self);
        self.iter_adjacent(hm.width, hm.len())
            .map(|x| hm.get(&x))
            .all(|x| h < x)
    }

    pub fn is_basin(&self, hm: &Heightmap) -> bool {
        hm.get(self) < 9
    }
    pub fn iter_adjacent(&self, width: usize, len: usize)
        -> impl Iterator<Item=Point>
    {
        let n = !self.is_north(width, len);
        let w = !self.is_west(width, len);
        let s = !self.is_south(width, len);
        let e = !self.is_east(width, len);
        let i = self.0;
        (0..4).into_iter()
            .filter_map(move |di| {
                match di {
                    0 if n => Some(Point(i - width)),
                    1 if w => Some(Point(i + 1)),
                    2 if s => Some(Point(i + width)),
                    3 if e => Some(Point(i - 1)),
                    _ => None,
                }
            })
    }
}

#[cfg(test)]
mod test {
    use crate::day9::main;
    use crate::day9::Heightmap;
    use crate::day9::Point;

    static DATA: &str = concat!(
        "2199943210\n",
        "3987894921\n",
        "9856789892\n",
        "8767896789\n",
        "9899965678",
    );

    #[test]
    fn part1() {
        let (p1, p2) = main(DATA).expect("invalid data");
        assert_eq!(p1, 15);
        assert_eq!(p2, 1134);
    }

    #[test]
    fn heightmap() {
        let hm: Heightmap = DATA.parse().expect("invalid data");
        assert_eq!(hm.len(), 5*10);

        let x = Point(0);
        assert!(x.is_north(hm.width, hm.len()));
        assert!(x.is_east(hm.width, hm.len()));
        assert!(!x.is_south(hm.width, hm.len()));
        assert!(!x.is_west(hm.width, hm.len()));

        let x = Point(9);
        assert!(x.is_north(hm.width, hm.len()));
        assert!(!x.is_east(hm.width, hm.len()));
        assert!(!x.is_south(hm.width, hm.len()));
        assert!(x.is_west(hm.width, hm.len()));

        let x = Point(10);
        assert!(!x.is_north(hm.width, hm.len()));
        assert!(x.is_east(hm.width, hm.len()));
        assert!(!x.is_south(hm.width, hm.len()));
        assert!(!x.is_west(hm.width, hm.len()));

        let x = Point(19);
        assert!(!x.is_north(hm.width, hm.len()));
        assert!(!x.is_east(hm.width, hm.len()));
        assert!(!x.is_south(hm.width, hm.len()));
        assert!(x.is_west(hm.width, hm.len()));

        let x = Point(40);
        assert!(!x.is_north(hm.width, hm.len()));
        assert!(x.is_east(hm.width, hm.len()));
        assert!(x.is_south(hm.width, hm.len()));
        assert!(!x.is_west(hm.width, hm.len()));

        let x = Point(41);
        assert!(!x.is_north(hm.width, hm.len()));
        assert!(!x.is_east(hm.width, hm.len()));
        assert!(x.is_south(hm.width, hm.len()));
        assert!(!x.is_west(hm.width, hm.len()));

        let x = Point(49);
        assert!(!x.is_north(hm.width, hm.len()));
        assert!(!x.is_east(hm.width, hm.len()));
        assert!(x.is_south(hm.width, hm.len()));
        assert!(x.is_west(hm.width, hm.len()));

        let x = Point(13);
        assert!(!x.is_north(hm.width, hm.len()));
        assert!(!x.is_east(hm.width, hm.len()));
        assert!(!x.is_south(hm.width, hm.len()));
        assert!(!x.is_west(hm.width, hm.len()));

        assert!(Point(1).is_minimum(&hm));
    }
}
