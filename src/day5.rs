use std::str::FromStr;
use std::collections::HashMap;
use std::cmp;
use std::ops;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let data = parse_input(input)?;
    let mut map1: HashMap<Point, usize> = HashMap::new();
    let mut map2: HashMap<Point, usize> = HashMap::new();

    for segm in &data {
        if segm.is_diagonal() {
            for p in segm.iter_points() {
                *map2.entry(p).or_insert(0) += 1;
            }
            continue
        }
        for p in segm.iter_points() {
            *map1.entry(p).or_insert(0) += 1;
            *map2.entry(p).or_insert(0) += 1;
        }
    }

    let result1 = map1.iter().filter(|&(_, x)| *x > 1).count();
    let result2 = map2.iter().filter(|&(_, x)| *x > 1).count();
    Ok((result1, result2))
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
