use advent::day4::{Table, Result};

static DATA: &str = include_str!("../input/day4.txt");

fn main() {
    println!("Day4, part1");

    let mut it = DATA.split_terminator("\n\n");
    let input = it.next().expect("no input numbers");

    let mut tables: Vec<Table> = it
        .map(|s| s.parse::<Table>().expect("invalid table data"))
        .collect();

    'outer:
    for num in input
        .split(',')
        .map(|s| s.parse::<usize>().expect("invalid number"))
    {
        for tbl in tables.iter_mut() {
            match tbl.add(num) {
                Result::Bingo(score) => {
                    println!("Bingo!");
                    println!("Number is {} and score {}", num, score);
                    println!("Answer: {}", num * score);
                    break 'outer
                }
                Result::Nope => (),
            }
        }
    }
}
