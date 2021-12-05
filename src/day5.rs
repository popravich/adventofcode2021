use std::str::FromStr;
use std::collections::HashSet;
use std::cmp;
use std::ops;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let data = parse_input(input)?;

    let mut overlapping1 = HashSet::new();
    let mut overlapping2 = HashSet::new();

    let data1: Vec<_> = data
        .iter()
        .filter(|s| s.is_horizontal() || s.is_vertical())
        .collect();

    for (i, l1) in data1.iter().take(data.len() - 1).enumerate() {
        for l2 in data1.iter().skip(i + 1) {
            overlapping1.extend(l1.overlapping_points(l2))
        }
    }

    let data2: Vec<_> = data
        .iter()
        .filter(|s| s.is_horizontal() || s.is_vertical() || s.is_diagonal())
        .collect();
    for (i, l1) in data2.iter().take(data.len() - 1).enumerate() {
        for l2 in data.iter().skip(i + 1) {
            overlapping2.extend(l1.overlapping_points(l2))
        }
    }
    Ok((overlapping1.len(), overlapping2.len()))
}

pub fn parse_input(input: &str) -> Result<Vec<Segment>, String> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.parse::<Segment>()?);
    }
    Ok(result)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Vertical {
    x: isize,
    y1: isize,
    y2: isize,
}

#[derive(Debug)]
pub struct Horizontal {
    x1: isize,
    x2: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Diagonal {
    from: Point,
    to: Point,
}

#[derive(Debug)]
pub enum Segment {
    Horizontal(Horizontal),
    Vertical(Vertical),
    Diagonal(Diagonal),
}

pub trait Contains {
    fn contains(&self, point: &Point) -> bool;
}


impl FromStr for Segment {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val.split_once(" -> ") {
            Some((p1, p2)) => {
                let p1: Point = p1.parse().map_err(|e| format!("{}", e))?;
                let p2: Point = p2.parse().map_err(|e| format!("{}", e))?;
                let from = cmp::min(p1, p2);
                let to = cmp::max(p1, p2);
                let seg = match (from.x == to.x, from.y == to.y) {
                    (true, true) => return Err("point is not supported".to_string()),
                    (true, false) => Segment::Vertical(Vertical {
                        x: from.x,
                        y1: from.y,
                        y2: to.y,
                    }),
                    (false, true) => Segment::Horizontal(Horizontal {
                        x1: from.x,
                        x2: to.x,
                        y: from.y,
                    }),
                    (false, false) => {
                        let k = (to.y - from.y) / (to.x - from.x);
                        if k.abs() == 1 {
                            Segment::Diagonal(Diagonal{
                                from, to,
                            })
                        } else {
                            return Err(format!("Other k: {}", k));
                        }
                    },
                };
                Ok(seg)
            }
            None => Err(format!("Invalid line data: {}", val)),
        }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val.split_once(',') {
            Some((x, y)) => {
                Ok(Point {
                    x: x.parse().map_err(|e| format!("{}", e))?,
                    y: y.parse().map_err(|e| format!("{}", e))?,
                })
            }
            None => Err(format!("Invalid point data: {}", val)),
        }
    }
}

impl Segment {

    pub fn is_horizontal(&self) -> bool {
        matches!(self, Segment::Horizontal(_))
    }
    pub fn is_vertical(&self) -> bool {
        matches!(self, Segment::Vertical(_))
    }
    pub fn is_diagonal(&self) -> bool {
        matches!(self, Segment::Diagonal(_))
    }

    pub fn overlapping_points<'a>(&'a self, other: &Self) -> impl Iterator<Item=Point> + 'a {
        // println!("XXXXXX {:?}, P: {:?}: {:?}", self, other.iter_points().collect::<Vec<_>>(),
        //     other.iter_points().filter(|p| self.contains(p)).collect::<Vec<_>>());
        other.iter_points().filter(|p| self.contains(p))
    }

    pub fn contains(&self, p: &Point) -> bool {
        use self::Segment::*;
        match self {
            Horizontal(h) => h.contains(p),
            Vertical(v) => v.contains(p),
            Diagonal(d) => d.contains(p),
        }
    }

    pub fn iter_points(&self) -> impl Iterator<Item=Point> {
        use self::Segment::*;
        match self {
            Horizontal(x) => Either::A(Either::A(x.iter_points())),
            Vertical(x) => Either::A(Either::B(x.iter_points())),
            Diagonal(x) => Either::B(x.iter_points()),
        }
    }
}

impl Horizontal {
    fn iter_points(&self) -> impl Iterator<Item=Point> {
        let y = self.y;
        (self.x1..=self.x2)
            .into_iter()
            .map(move |x| Point { x, y })
    }
}
impl Vertical {
    fn iter_points(&self) -> impl Iterator<Item=Point> {
        let x = self.x;
        (self.y1..=self.y2)
            .into_iter()
            .map(move |y| Point { x, y })
    }
}
impl Diagonal {
    fn iter_points(&self) -> impl Iterator<Item=Point> {
        let is_reverse = self.to.y < self.from.y;
        let it = if is_reverse {
            Either::A(ops::RangeInclusive::new(self.to.y, self.from.y).rev())
        } else {
            Either::B(ops::RangeInclusive::new(self.from.y, self.to.y))
        };
        (self.from.x..=self.to.x)
            .into_iter()
            .zip(it)
            .map(|(x, y)| Point { x, y })
    }
}

impl Contains for Horizontal {
    fn contains(&self, p: &Point) -> bool {
        self.y == p.y && (self.x1..=self.x2).contains(&p.x)
    }
}
impl Contains for Vertical {
    fn contains(&self, p: &Point) -> bool {
        self.x == p.x && (self.y1..=self.y2).contains(&p.y)
    }
}
impl Contains for Diagonal {
    fn contains(&self, p: &Point) -> bool {
        if !(self.from.x..=self.to.x).contains(&p.x) {
            return false
        }
        let k = (self.to.y - self.from.y) / (self.to.x - self.from.x);
        let b = self.from.y - (k * self.from.x);
        // println!("P = {:?}", p);
        // println!("F(x) = {} * x + {} = {}", k, b, self.from.y);
        let y = (k * p.x) + b;
        // println!("F(p) = {} == {}", y, y == p.y);
        y == p.y
        // let contains_x = p.x >= self.from.x && p.x <= self.to.x;
        // if let Kind::Diagonal(k) = self.kind() {
        //     let b = self.from.y - (k * self.from.x);
        //     let y = p.x * k + b;
        //     contains_x && y == p.y
        // } else {
        //     contains_x && p.y >= self.from.y && p.y <= self.to.y
        // }
    }
}

enum Either<T, U> {
    A(T),
    B(U),
}

impl<I, T, U> Iterator for Either<T, U>
    where T: Iterator<Item=I>,
        U: Iterator<Item=I>,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::A(it) => it.next(),
            Either::B(it) => it.next(),
        }
    }
}
