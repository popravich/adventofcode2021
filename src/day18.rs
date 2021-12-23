use std::str::FromStr;
use std::fmt;
use std::cmp;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let mut new = Vec::new();
    for line in input.lines() {
        new.push(parse_str(line));
    }

    let mut result = new[0].clone();
    for next in &new[1..] {
        result = reduce(add(&result, &next));
    }
    let part1 = magnitude(&result);

    let mut part2 = 0;
    for i in 0..new.len() - 1 {
        for j in i+1..new.len() {
            let a = magnitude(&reduce(add(&new[i], &new[j])));
            let b = magnitude(&reduce(add(&new[j], &new[i])));
            part2 = cmp::max(part2, cmp::max(a, b));
        }
    }
    Ok((part1, part2))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnailfishNumber(pub Pair0, pub Pair0);


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pair3 {
    Number(usize),
    Pair(usize, usize),
}

pub trait SnailfishMath: Sized {
    fn combine(self, other: Self) -> Self;
    fn magnitude(&self) -> usize;

    fn need_explode(&self) -> bool;
    fn explode(self) -> Self;

    fn need_split(&self) -> bool;
    fn split(self) -> Self;

    fn add(self, other: Self) -> Self {
        let mut next = self.combine(other);
        loop {
            while next.need_explode() {
                next = next.explode();
            }
            while next.need_split() && !next.need_explode() {
                next = next.split();
            }
            if !next.need_explode() && !next.need_split() {
                break
            }
        }
        next
    }

}

impl SnailfishMath for SnailfishNumber {
    fn combine(self, other: Self) -> Self {
        SnailfishNumber(self.into(), other.into())
    }

    fn magnitude(&self) -> usize {
        self.0.magnitude() * 3 + self.1.magnitude() * 2
    }

    fn need_explode(&self) -> bool {
        self.0.need_explode() || self.1.need_explode()
    }

    fn explode(self) -> Self {
        if self.0.need_explode() {
            let (a, b) = match self.0.explode() {
                Explode::None(a) => (a, self.1),
                Explode::Left(_, a) => (a, self.1),
                Explode::Right(a, x) => (a, self.1.add_left(x)),
            };
            SnailfishNumber(a, b)
        } else if self.1.need_explode() {
            let (a, b) = match self.1.explode() {
                Explode::None(b) => (self.0, b),
                Explode::Left(x, b) => (self.0.add_right(x), b),
                Explode::Right(b, _) => (self.0, b),
            };
            SnailfishNumber(a, b)
        } else {
            self
        }
    }

    fn need_split(&self) -> bool {
        self.0.need_split() || self.1.need_split()
    }

    fn split(self) -> Self {
        if self.0.need_split() {
            SnailfishNumber(self.0.split(), self.1)
        } else if self.1.need_split() {
            SnailfishNumber(self.0, self.1.split())
        } else {
            self
        }
    }
}
// No errors, just unwrap everything;

impl FromStr for SnailfishNumber {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let next = val
            .strip_prefix('[')
            .expect("invalid suffix");
        let a: Pair0 = next.parse()?;
        let mid = val.match_indices(',')
            .skip(a.num_pairs())
            .next()
            .map(|(i, _)| i + 1)
            .expect("no separator");
        let (_, next) = val.split_at(mid);
        let b = next.parse()?;
        Ok(SnailfishNumber(a, b))
    }
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}
macro_rules! define_pair {
    ($name:ident($inner:ident)) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum $name {
            Number(usize),
            Pair($inner, $inner),
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(val: &str) -> Result<Self, Self::Err> {
                use self::$name::*;
                let (left, _) = val.split_once(|c| c == ',' || c == ']')
                    .expect("no separators");
                match left.chars().next().expect("empty string") {
                    c if c.is_digit(10) => {
                        let v: usize = left.parse()
                            .expect("invalid number");
                        Ok(Number(v))
                    }
                    '[' => {
                        let sub = val.strip_prefix('[').expect("invalid prefix");
                        let a: $inner = sub.parse()?;
                        let mid = val.match_indices(',')
                            .skip(a.num_pairs())
                            .next()
                            .map(|(i, _)| i)
                            .expect("no separator");
                        let (_, sub) = sub.split_at(mid);
                        let b = sub.parse()?;
                        Ok(Pair(a, b))
                    }
                    x => unreachable!("unexpected char: {}: {}", x, val),
                }
            }
        }
        impl From<usize> for $name {
            fn from(val: usize) -> Self {
                $name::Number(val)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $name::Number(x) => write!(f, "{}", x),
                    $name::Pair(a, b) => {
                        f.write_str("[")?;
                        a.fmt(f)?;
                        f.write_str(",")?;
                        b.fmt(f)?;
                        f.write_str("]")
                    }
                }
            }
        }

        impl $name {
            pub fn num_pairs(&self) -> usize {
                match self {
                    $name::Number(_) => 0,
                    $name::Pair(a, b) => 1 + a.num_pairs() + b.num_pairs(),
                }
            }

            fn add_left(self, val: usize) -> Self {
                match self {
                    $name::Number(v) => $name::Number(v + val),
                    $name::Pair(a, b) => $name::Pair(a.add_left(val), b),
                }
            }
            fn add_right(self, val: usize) -> Self {
                match self {
                    $name::Number(v) => $name::Number(v + val),
                    $name::Pair(a, b) => $name::Pair(a, b.add_right(val)),
                }
            }
        }
    };
}

macro_rules! impl_pair {
    (From<$outer:ident> for $inner:ident) => {
        impl From<$outer> for $inner {
            fn from(val: $outer) -> Self {
                match val {
                    $outer::Number(x) => $inner::Number(x),
                    $outer::Pair(a, b) => $inner::Pair(a.into(), b.into()),
                }
            }
        }
    };
    (explode $name:ident recursive) => {
        impl $name {    
            pub fn magnitude(&self) -> usize {
                match self {
                    $name::Number(x) => *x,
                    $name::Pair(a, b) => 3*a.magnitude() + 2*b.magnitude(),
                }
            }

            pub fn explode(self) -> Explode<Self> {
                match self {
                    $name::Number(_) => Explode::None(self),
                    $name::Pair(a, b) => {
                        let (a, b) = match a.explode() {
                            Explode::None(a) => (a, b),
                            Explode::Left(x, a) => {
                                return Explode::Left(x, $name::Pair(a, b))
                            }
                            Explode::Right(a, x) => (a, b.add_left(x)),
                        };
                        match b.explode() {
                            Explode::None(b) => {
                                Explode::None($name::Pair(a, b))
                            }
                            Explode::Left(x, b) => {
                                Explode::None($name::Pair(a.add_right(x), b))
                            }
                            Explode::Right(b, x) => {
                                Explode::Right($name::Pair(a, b), x)
                            }
                        }
                    }
                }
            }
            pub fn need_explode(&self) -> bool {
                match self {
                    $name::Number(_) => false,
                    $name::Pair(a, b) => a.need_explode() || b.need_explode(),
                }
            }
        }
    };
    (explode $name:ident with $inner:ident) => {
        impl $name {

            pub fn magnitude(&self) -> usize {
                match self {
                    $name::Number(x) => *x,
                    $name::Pair(a, b) => 3*a.magnitude() + 2*b.magnitude(),
                }
            }

            pub fn explode(self) -> Explode<Self> {
                match self {
                    $name::Pair($inner::Pair(a, b), c) => {
                        Explode::Left(
                            a, $name::Pair($inner::Number(0), c.add_left(b)),
                        )
                    }
                    $name::Pair(c, $inner::Pair(a, b)) => {
                        Explode::Right(
                            $name::Pair(c.add_right(a), $inner::Number(0)), b,
                        )
                    }
                    _ => Explode::None(self),
                }
            }
            pub fn need_explode(&self) -> bool {
                matches!(self,
                    $name::Pair($inner::Pair(_,_), _) |
                    $name::Pair(_, $inner::Pair(_,_)))
            }
        }
    };
    (split $name:ident recursive) => {
        impl $name {
            pub fn split(self) -> Self {
                match self {
                    $name::Number(x) => {
                        if x >= 10 {
                            let a = x / 2;
                            let b = x - a;
                            $name::Pair(a.into(), b.into())
                        } else {
                            $name::Number(x)
                        }
                    }
                    $name::Pair(a, b) => {
                        $name::Pair(a.split(), b.split())
                    }
                }
            }
            pub fn need_split(&self) -> bool {
                match self{
                    $name::Number(x) => *x >= 10,
                    $name::Pair(a, b) => a.need_split() || b.need_split(),
                }
            }
        }
    };
    (split $name:ident) => {
        impl $name {
            pub fn split(self) -> Self {
                match self {
                    $name::Number(x) => {
                        if x >= 10 {
                            $name::Pair(x / 2, x - (x / 2))
                        } else {
                            $name::Number(x)
                        }
                    }
                    $name::Pair(a, b) => $name::Pair(a, b),
                }
            }
            pub fn need_split(&self) -> bool {
                match self {
                    $name::Number(x) => *x >= 10,
                    $name::Pair(_, _) => false, // ignore this, explode first
                }
            }
        }
    };
}

define_pair!(Pair0(Pair1));
define_pair!(Pair1(Pair2));
define_pair!(Pair2(Pair3));
impl_pair!(From<Pair0> for Pair1);
impl_pair!(From<Pair1> for Pair2);
impl_pair!(explode Pair0 recursive);
impl_pair!(explode Pair1 recursive);
impl_pair!(explode Pair2 with Pair3);
impl_pair!(split Pair0 recursive);
impl_pair!(split Pair1 recursive);
impl_pair!(split Pair2 recursive);
impl_pair!(split Pair3);


impl FromStr for Pair3 {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let (left, _) = val.split_once(|c| c == ',' || c == ']')
            .expect("no separators");
        match left.chars().next().expect("empty string") {
            c if c.is_digit(10) => {
                let (sub, _) = val
                    .split_once(|c| c == ',' || c == ']')
                    .expect("no separator");
                let v: usize = sub.parse()
                    .expect("invalid number");
                Ok(Pair3::Number(v))
            }
            '[' => {
                let (left, right) = val.strip_prefix('[')
                    .and_then(|s| s.split_once(','))
                    .expect("no separator");
                let a = left.parse().expect("invalid number");
                let (right, _) = right.split_once(']')
                    .expect("no right bracket");
                let b = right.parse().expect("invalid number");
                Ok(Pair3::Pair(a, b))
            }
            x => unreachable!("unexpected char: {}: {}", x, val),
        }
    }
}

impl From<SnailfishNumber> for Pair0 {
    fn from(val: SnailfishNumber) -> Self {
        Pair0::Pair(val.0.into(), val.1.into())
    }
}
impl From<Pair2> for Pair3 {
    fn from(val: Pair2) -> Self {
        match val {
            Pair2::Number(x) => Pair3::Number(x),
            Pair2::Pair(Pair3::Number(a), Pair3::Number(b)) => {
                Pair3::Pair(a, b)
            }
            _ => unreachable!("too deep")
        }
    }
}

impl fmt::Display for Pair3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pair3::Number(x) => write!(f, "{}", x),
            Pair3::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

impl From<usize> for Pair3 {
    fn from(val: usize) -> Self {
        Pair3::Number(val)
    }
}

impl SnailfishNumber {
    pub fn num_pairs(&self) -> usize {
        self.0.num_pairs() + self.1.num_pairs()
    }
}

impl Pair3 {

    pub fn magnitude(&self) -> usize {
        match self {
            Pair3::Number(x) => *x,
            Pair3::Pair(a, b) => 3*a + 2*b,
        }
    }
    pub fn num_pairs(&self) -> usize {
        use self::Pair3::*;
        match self {
            Number(_) => 0,
            Pair(_, _) => 1,
        }
    }

    fn add_left(self, val: usize) -> Self {
        match self {
            Pair3::Number(v) => Pair3::Number(v + val),
            Pair3::Pair(a, b) => Pair3::Pair(a + val, b),
        }
    }
    fn add_right(self, val: usize) -> Self {
        match self {
            Pair3::Number(v) => Pair3::Number(v + val),
            Pair3::Pair(a, b) => Pair3::Pair(a, b + val),
        }
    }

}

#[derive(Debug)]
pub enum Explode<T> {
    None(T),
    Left(usize, T),
    Right(T, usize),
}

#[cfg(test)]
mod test {
    use crate::day18::*;

    static DATA: &str = concat!(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]\n",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n",
        "[[[[5,4],[7,7]],8],[[8,3],8]]\n",
        "[[9,3],[[9,9],[6,[4,9]]]]\n",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    );

    #[test]
    fn solution() {
        let (p1, p2) = main(DATA).expect("invalid input");
        assert_eq!(p1, 4140);
        assert_eq!(p2, 3993);
    }

    #[test]
    fn parse_pair3() {
        let a: Pair3 = "1,".parse().expect("invalid input");
        assert_eq!(a, Pair3::Number(1));
        assert_eq!(a.num_pairs(), 0);

        let a: Pair3 = "[1,2]".parse().expect("invalid input");
        assert_eq!(a, Pair3::Pair(1, 2));
        assert_eq!(a.num_pairs(), 1);
    }

    #[test]
    fn parse_pair2() {
        let a: Pair2 = "1,".parse().expect("invalid input");
        assert_eq!(a, Pair2::Number(1));
        assert_eq!(a.num_pairs(), 0);

        let a: Pair2 = "[1,2]".parse().expect("invalid input");
        assert_eq!(a, Pair2::Pair(Pair3::Number(1), Pair3::Number(2)));
        assert_eq!(a.num_pairs(), 1);

        let a: Pair2 = "[1,[2,3]]".parse().expect("invalid input");
        assert_eq!(a, Pair2::Pair(Pair3::Number(1), Pair3::Pair(2, 3)));
        assert_eq!(a.num_pairs(), 2);

        let a: Pair2 = "[[0,1],[2,3]]".parse().expect("invalid input");
        assert_eq!(a, Pair2::Pair(Pair3::Pair(0, 1), Pair3::Pair(2, 3)));
        assert_eq!(a.num_pairs(), 3);

        let a: Pair2 = "[[0,1],2]".parse().expect("invalid input");
        assert_eq!(a, Pair2::Pair(Pair3::Pair(0, 1), Pair3::Number(2)));
        assert_eq!(a.num_pairs(), 2);
    }

    #[test]
    fn operations() {
        let a: SnailfishNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]"
            .parse()
            .expect("invalid number");
        let b: SnailfishNumber = "[1,1]"
            .parse()
            .expect("invalid number");
        let c = a.clone().combine(b.clone());
        assert_eq!(c, "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap());
        let d = c.explode();
        assert_eq!(d, "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap());
        let e = d.split();
        assert_eq!(e, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".parse().unwrap());

        let f = a.add(b);
        assert_eq!(f, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap());
        assert_eq!(f.magnitude(), 1384);
    }

    #[test]
    fn example1() {
        let data = concat!(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n",
            "[7,[5,[[3,8],[1,4]]]]\n",
            "[[2,[2,2]],[8,[8,1]]]\n",
            "[2,9]\n",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]\n",
            "[[[5,[7,4]],7],1]\n",
            "[[[[4,2],2],6],[8,7]]",
        );
        let mut numbers: Vec<SnailfishNumber> = Vec::new();
        for line in data.lines() {
            numbers.push(line.parse().expect("not a valid number"));
        }
        let result = dbg!(numbers.remove(0));
        let result = result.add(numbers.remove(0));
        assert_eq!(
            format!("{}", result),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );

    }
}

#[derive(Debug, Clone)]
pub struct Item {
    level: usize,
    value: usize,
}


pub fn parse_str(input: &str) -> Vec<Item> {
    let mut rv = Vec::new();
    let mut level = 0;
    let mut value = None;
    for c in input.chars() {
        match c {
            '[' => level += 1,
            ']' => {
                if let Some(value) = value.take() {
                    rv.push(Item { level, value });
                }
                level -= 1;
            }
            ',' => {
                if let Some(value) = value.take() {
                    rv.push(Item { level, value });
                }
            }
            x if x.is_digit(10) => {
                let x = x.to_digit(10).unwrap() as usize;
                value = Some(value.unwrap_or_default() * 10 + x);
            }
            _ => unreachable!("unexpected char: {}", c),
        }
    }
    rv
}

#[derive(Debug)]
pub enum Action {
    Explode {
        at: usize,
        left: usize,
        right: usize,
    },
    Split {
        at: usize,
        level: usize,
        left: usize,
        right: usize,
    },
}

pub fn scan_number(input: &[Item]) -> Option<Action> {
    let mut it = input.iter().enumerate();
    let mut split = None;
    while let Some((idx, item)) = it.next() {
        if item.level == 5 {
            let (_, next) = it.next().expect("no neighbour");
            assert_eq!(next.level, 5, "Data: {:?} {:?}", idx, input);
            return Some(Action::Explode {
                at: idx,
                left: item.value,
                right: next.value,
            })
        } else if item.value >= 10 && split.is_none() {
            split = Some(Action::Split {
                at: idx,
                level: item.level + 1,
                left: item.value / 2,
                right: item.value - (item.value / 2),
            });
        }
    }
    split
}

pub fn add(left: &[Item], right: &[Item]) -> Vec<Item> {
    left.iter()
        .chain(right)
        .map(|i| {
            let mut i = i.clone();
            i.level += 1;
            i
        })
        .collect()
}

pub fn reduce(mut data: Vec<Item>) -> Vec<Item> {
    while let Some(action) = scan_number(&data) {
        match action {
            Action::Explode { at, left, right } => {
                data.remove(at + 1);
                data[at].level = 4;
                data[at].value = 0;
                if at > 0 {
                    data[at - 1].value += left;
                }
                if at + 1 < data.len() {
                    data[at + 1].value += right;
                }
            }
            Action::Split { at, level, left, right } => {
                data[at].level = level;
                data[at].value = right;
                data.insert(at, Item { level, value: left });
            }
        }
    }
    data
}

pub fn magnitude(data: &[Item]) -> usize {
    let mut tmp1 = data.to_vec();
    let mut tmp2 = Vec::with_capacity(tmp1.len());
    while tmp1.len() != 1 {
        let mut it = tmp1.into_iter();
        let mut prev = it.next().expect("empty data");
        while let Some(next) = it.next() {
            if prev.level == next.level {
                tmp2.push(Item {
                    level: prev.level - 1,
                    value: 3 * prev.value + 2 * next.value,
                });
                prev = match it.next() {
                    Some(p) => p,
                    None => break,
                };
            } else {
                tmp2.push(prev.clone());
                prev = next;
            }
        }
        tmp1 = tmp2.drain(..).collect();
    };
    tmp1[0].value
}
