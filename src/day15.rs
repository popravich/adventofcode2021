use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use crossterm::style::Stylize;


pub fn main(input: &str) -> Result<(usize, usize), String> {
    let map: Map = input.parse()?;

    let mut path = build_path(&map);

    let goal = Point { x: map.side - 1, y: map.side - 1};
    let mut part1 = map.score(&goal);
    let mut prev = goal.clone();
    let mut result = HashSet::new();
    result.insert(goal);
    while let Some(p) = path.remove(&prev) {
        part1 += map.score(&p);
        result.insert(p.clone());
        prev = p;
    }
    part1 -= map.score((0, 0));

    for i in 0..map.side {
        for k in 0..map.side {
            if result.contains(&Point { x: k, y: i }) {
                print!("{}", format!("{}", map.score((k, i))).red());
            } else {
                print!("{}", map.score((k, i)));
            }
        }
        println!("");
    }

    let mut map2 = Map {
        side: map.side * 5,
        data: Vec::with_capacity(map.side * map.side * 25),
    };
    for y in 0..map2.side {
        for x in 0..map2.side {
            let x0 = x % map.side;
            let y0 = y % map.side;
            let k = x / map.side + y / map.side;
            let mut s0 = map.score((x0, y0)) + k;
            s0 = s0 % 9;
            // println!("xxxxx {} {}; {} {}", x,y, s0, map.score(&Point { x: x0, y: y0 }));
            if s0 == 0 {
                s0 = 9;
            }
            map2.data.push(s0);
        }
    }

    let mut path = build_path(&map2);
    let goal = Point { x: map2.side - 1, y: map2.side - 1};
    let mut part2 = map2.score(&goal);
    let mut prev = goal.clone();
    let mut result = HashSet::new();
    result.insert(goal);
    while let Some(p) = path.remove(&prev) {
        part2 += map2.score(&p);
        result.insert(p.clone());
        prev = p;
    }
    part2 -= map2.score((0, 0));

    for i in 0..map2.side {
        if i % map.side == 0 {
            for _ in 0..map2.side { print!("-"); }
            println!("");
        }
        for k in 0..map2.side {
            if k % map.side == 0 {
                print!("|");
            }
            if result.contains(&Point { x: k, y: i }) {
                print!("{}", format!("{}", map2.score((k, i))).red());
            } else {
                print!("{}", map2.score((k, i)));
            }
        }
        println!("");
    }

    Ok((part1, part2))
}

pub fn pop_lowest(
    set: &HashSet<Point>, scores: &HashMap<Point, usize>,
) -> Point {
    set.iter()
        .min_by_key(|p| scores.get(p).expect("no score"))
        .expect("empty set")
        .clone()
}

pub fn build_path(map: &Map) -> HashMap<Point, Point> {
    let start = Point { x: 0, y: 0 };
    let goal = Point { x: map.side - 1, y: map.side - 1};

    let mut todo = HashSet::new();
    todo.insert(start.clone());

    let mut f_scores = HashMap::new();
    f_scores.insert(start.clone(), 0);

    let mut g_scores = HashMap::new();
    g_scores.insert(start.clone(), 0);

    let mut path = HashMap::new();
    while !todo.is_empty() {
        let current = pop_lowest(&todo, &f_scores);
        todo.remove(&current);

        if current == goal {
            break
        }
        // println!("checking neighbours of {:?}", current);
        for n in current.neighbours(map.side) {
            // print!("   {:?}", n);
            let score = g_scores
                .get(&current)
                .map(|s| s + map.score(&n))
                .unwrap_or(usize::MAX);
            if score < *g_scores.get(&n).unwrap_or(&usize::MAX) {
                path.insert(n.clone(), current.clone());
                g_scores.insert(n.clone(), score);
                f_scores.insert(n.clone(), score);
                todo.insert(n);
            }
        }
    }
    path
}


#[derive(Debug, Default)]
pub struct Map {
    side: usize,
    data: Vec<usize>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let side = val.lines().count();
        let mut data = Vec::with_capacity(side * side);
        for c in val.lines().flat_map(|l| l.chars()) {
            let c = c as u8;
            if c >= b'0' && c <= b'9' {
                data.push((c - b'0') as usize);
            } else {
                return Err(format!("unexpected digit {}", c));
            }
        }
        Ok(Map { side, data })
    }
}

impl Map {
    pub fn score<O: ByteOffset>(&self, o: O) -> usize {
        let idx = o.offset(self.side);
        self.data[idx]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbours(&self, side: usize) -> impl Iterator<Item=Point> {
        let x = self.x as isize;
        let y = self.y as isize;
        let s = side as isize;
        [(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)]
            .into_iter()
            .filter(move |&(x, y)| x >= 0 && x < s && y >= 0 && y < s)
            .map(|(x, y)| Point { x: x as usize, y: y as usize })
    }
}

pub trait ByteOffset {
    fn offset(&self, square_side: usize) -> usize;
}

impl ByteOffset for &Point {
    fn offset(&self, square_side: usize) -> usize {
        self.y * square_side + self.x
    }
}

impl ByteOffset for (usize, usize) {
    fn offset(&self, square_side: usize) -> usize {
        self.1 * square_side + self.0
    }
}

#[cfg(test)]
mod test {

    use crate::day15::main;

    static DATA: &str = concat!(
        "1163751742\n",
        "1381373672\n",
        "2136511328\n",
        "3694931569\n",
        "7463417111\n",
        "1319128137\n",
        "1359912421\n",
        "3125421639\n",
        "1293138521\n",
        "2311944581",
    );

    #[test]
    fn solution() {
        let (p1, p2) = main(DATA).expect("invalid input");
        assert_eq!(p1, 40);
        assert_eq!(p2, 315);
    }
}
