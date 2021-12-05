use std::str::FromStr;
use std::collections::HashSet;
use std::cmp;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let mut data = parse_input(input)?;
    data.retain(|l| l.is_horizontal() || l.is_vertical());
    assert!(data.iter().all(|l| l.is_horizontal() || l.is_vertical()));
    let mut overlapping = HashSet::new();
    for (i, l1) in data.iter().take(data.len() - 1).enumerate() {
        for l2 in data.iter().skip(i + 1) {
            overlapping.extend(l1.overlapping_points(l2))
        }
    }
    Ok((overlapping.len(), 0))
}

pub fn parse_input(input: &str) -> Result<Vec<Segment>, String> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.parse::<Segment>()?);
    }
    Ok(result)
}

#[derive(Debug)]
pub struct Segment {
    from: Point,
    to: Point,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub enum Axis {
    Horizontal,
    Vertical,
}


impl FromStr for Segment {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val.split_once(" -> ") {
            Some((p1, p2)) => {
                let p1 = p1.parse().map_err(|e| format!("{}", e))?;
                let p2 = p2.parse().map_err(|e| format!("{}", e))?;
                let from = cmp::min(p1, p2);
                let to = cmp::max(p1, p2);
                Ok(Segment {
                    from,
                    to,
                })
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
    pub fn is_axis(&self) -> Option<Axis> {
        match (self.from.x == self.to.x, self.from.y == self.to.y) {
            (true, true) => unreachable!("Point not a segment"),
            (true, false) => Some(Axis::Vertical),
            (false, true) => Some(Axis::Horizontal),
            (false, false) => None,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        matches!(self.is_axis(), Some(Axis::Horizontal))
    }
    pub fn is_vertical(&self) -> bool {
        matches!(self.is_axis(), Some(Axis::Vertical))
    }

    pub fn overlapping_points<'a>(&'a self, other: &Self) -> impl Iterator<Item=Point> + 'a {
        other.iter_points().filter(|p| self.contains(p))
    }

    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.from.x && p.x <= self.to.x && p.y >= self.from.y && p.y <= self.to.y
    }

    pub fn iter_points(&self) -> impl Iterator<Item=Point> {
        let x = self.from.x;
        let y = self.from.y;
        if self.is_horizontal() {
            Either::A((x..=self.to.x)
                .into_iter()
                .map(move |x| Point { x, y }))
        } else {
            Either::B((y..=self.to.y)
                .into_iter()
                .map(move |y| Point { x, y }))
        }
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
