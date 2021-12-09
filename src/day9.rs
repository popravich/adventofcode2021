use std::str::FromStr;
use std::collections::HashSet;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let heights: Heightmap = input.parse()?;

    let mut i = 0;
    let mut extremums = Vec::new();
    while i < heights.len() {
        let p = Point(i);
        if p.is_minimum(&heights) {
            extremums.push((p.0, heights.get(&p) as usize));
            i += 2;
            continue
        }
        i += 1;
    }

    let result1 = extremums.iter().map(|&(_, h)| h).sum::<usize>() + extremums.len();


    let mut top_basins = [0usize; 4];

    let mut visited = HashSet::new();
    let mut to_check = Vec::new();

    for p in extremums.iter().map(|&(i, _)| Point(i)) {
        assert!(p.is_basin(&heights));
        visited.clear();
        to_check.clear();
        to_check.push(p);
        visited.insert(p);
        let mut size = 1;
        while !to_check.is_empty() {
            let p = to_check.remove(0);
            for x in p.iter_adjacent(&heights).filter(|x| !visited.contains(x)) {
                if x.is_basin(&heights) {
                    to_check.push(x);
                    size += 1;
                }
            }
            visited.extend(&to_check);
        }
        top_basins[0] = size;
        top_basins.sort();
    }


    println!("Sizes: {:?}", top_basins);
    let result2 = top_basins[1..].iter().fold(1, |a, b| a * b);
    Ok((result1, result2))
}


#[derive(Debug, Default)]
pub struct Heightmap {
    width: usize,
    data: Vec<u8>,
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
pub struct Point(usize);

impl Point {
    pub fn is_north(&self, hm: &Heightmap) -> bool {
        self.0 < hm.width
    }
    pub fn is_south(&self, hm: &Heightmap) -> bool {
        self.0 >= hm.data.len() - hm.width
    }
    pub fn is_east(&self, hm: &Heightmap) -> bool {
        self.0 % hm.width == 0
    }
    pub fn is_west(&self, hm: &Heightmap) -> bool {
        self.0 % hm.width == hm.width - 1
    }

    pub fn is_minimum(&self, hm: &Heightmap) -> bool {
        let h = hm.get(self);
        self.iter_adjacent(hm)
            .map(|x| hm.get(&x))
            .all(|x| h < x)
    }

    pub fn is_basin(&self, hm: &Heightmap) -> bool {
        hm.get(self) < 9
    }
    pub fn iter_adjacent(&self, hm: &Heightmap) -> impl Iterator<Item=Point> {
        let n = !self.is_north(hm);
        let w = !self.is_west(hm);
        let s = !self.is_south(hm);
        let e = !self.is_east(hm);
        let i = self.0;
        let width = hm.width;
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
        assert!(x.is_north(&hm));
        assert!(x.is_east(&hm));
        assert!(!x.is_south(&hm));
        assert!(!x.is_west(&hm));

        let x = Point(9);
        assert!(x.is_north(&hm));
        assert!(!x.is_east(&hm));
        assert!(!x.is_south(&hm));
        assert!(x.is_west(&hm));

        let x = Point(10);
        assert!(!x.is_north(&hm));
        assert!(x.is_east(&hm));
        assert!(!x.is_south(&hm));
        assert!(!x.is_west(&hm));

        let x = Point(19);
        assert!(!x.is_north(&hm));
        assert!(!x.is_east(&hm));
        assert!(!x.is_south(&hm));
        assert!(x.is_west(&hm));

        let x = Point(40);
        assert!(!x.is_north(&hm));
        assert!(x.is_east(&hm));
        assert!(x.is_south(&hm));
        assert!(!x.is_west(&hm));

        let x = Point(41);
        assert!(!x.is_north(&hm));
        assert!(!x.is_east(&hm));
        assert!(x.is_south(&hm));
        assert!(!x.is_west(&hm));

        let x = Point(49);
        assert!(!x.is_north(&hm));
        assert!(!x.is_east(&hm));
        assert!(x.is_south(&hm));
        assert!(x.is_west(&hm));

        let x = Point(13);
        assert!(!x.is_north(&hm));
        assert!(!x.is_east(&hm));
        assert!(!x.is_south(&hm));
        assert!(!x.is_west(&hm));

        assert!(Point(1).is_minimum(&hm));
    }
}
