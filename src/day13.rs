use std::str::FromStr;
use std::collections::HashSet;

pub fn main(input: &str) -> Result<(usize, usize), String> {
    let (dots, folds): (Vec<Dot>, Vec<Fold>) = {
        let (a, b) = input.split_once("\n\n").expect("invalid data");
        let mut dots = Vec::new();
        for line in a.lines() {
            dots.push(line.parse()?);
        }
        let mut folds = Vec::new();
        for line in b.lines() {
            folds.push(line.parse()?);
        };
        (dots, folds)
    };

    let part1 = dots
        .iter()
        .map(|d| d.translate(&folds[0]))
        .collect::<HashSet<_>>();

    let mut part2 = dots.into_iter().collect::<HashSet<_>>();
    for f in folds {
        part2 = part2
            .into_iter()
            .map(|d| d.translate(&f))
            .collect()
    }


    let max_x = part2.iter().map(|d| d.x).max().expect("no data");
    let max_y = part2.iter().map(|d| d.y).max().expect("no data");

    for y in 0..=max_y {
        for x in 0..=max_x {
            let d = Dot { x, y };
            if part2.contains(&d) {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    Ok((part1.len(), 0))
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Dot {
    x: usize,
    y: usize,
}

impl FromStr for Dot {
    type Err = String;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let (x, y) = val.split_once(',').expect("missing separator");
        let x = x.parse().map_err(|e| format!("{}", e))?;
        let y = y.parse().map_err(|e| format!("{}", e))?;
        Ok(Dot { x, y })
    }
}

impl Dot {
    pub fn translate(&self, fold: &Fold) -> Self {
        match fold {
            Fold::X(x) if self.x > *x => {
                Dot {
                    x: 2 * x - self.x,
                    y: self.y,
                }
            }
            Fold::Y(y) if self.y > *y => {
                Dot {
                    x: self.x,
                    y: 2 * y - self.y,
                }
            }
            _ => self.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Fold {
    Y(usize),
    X(usize),
}

impl FromStr for Fold {
    type Err = String;
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let (action, val) = val.split_once("=")
            .ok_or("Missing separator".to_string())?;
        let val = val.parse().map_err(|e| format!("{}", e))?;
        match action {
            "fold along y" => Ok(Fold::Y(val)),
            "fold along x" => Ok(Fold::X(val)),
            x => Err(format!("invalid fold: {}", x))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day13::main;
    use crate::day13::{Dot, Fold};

    static DATA: &str = concat!(
        "6,10\n",
        "0,14\n",
        "9,10\n",
        "0,3\n",
        "10,4\n",
        "4,11\n",
        "6,0\n",
        "6,12\n",
        "4,1\n",
        "0,13\n",
        "10,12\n",
        "3,4\n",
        "3,0\n",
        "8,4\n",
        "1,10\n",
        "2,14\n",
        "8,10\n",
        "9,0\n",
        "\n",
        "fold along y=7\n",
        "fold along x=5",
    );

    #[test]
    fn solution() {
        let (p1, p2) = main(DATA).expect("invalid data");
        assert_eq!(p1, 17);
        assert_eq!(p2, 0);
    }

    #[test]
    fn folds() {
        let a = Dot { x: 0, y: 2 };
        let b = a.translate(&Fold::Y(1));
        assert_eq!(b.x, 0);
        assert_eq!(b.y, 0);

        let a = Dot { x: 0, y: 0 };
        let b = a.translate(&Fold::Y(1));
        assert_eq!(b.x, 0);
        assert_eq!(b.y, 0);
    }
}
