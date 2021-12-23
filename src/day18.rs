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
    /// On explode:
    /// * remove next to `at` item;
    /// * replace `at` with 0 value and level 4;
    /// * incremenet left right neighbours if any;
    Explode {
        at: usize,
        left: usize,
        right: usize,
    },
    /// On split:
    /// * replace `at` item with `right` value and `level` level;
    /// * insert new item at `at` and set value to `left` and level to `level`;
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

}
