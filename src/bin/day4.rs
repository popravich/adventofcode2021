use advent::day4::{Table, Result};

static DATA: &str = include_str!("../input/day4.txt");

fn main() {
    println!("Day4");

    let mut it = DATA.split_terminator("\n\n");
    let input = it.next().expect("no input numbers");

    let mut tables: Vec<Table> = it
        .map(|s| s.parse::<Table>().expect("invalid table data"))
        .collect();

    let mut first_board = None;
    let mut last_board = None;
    for num in input
        .split(',')
        .map(|s| s.parse::<usize>().expect("invalid number"))
    {
        for (i, tbl) in tables
            .iter_mut()
            .enumerate()
            .filter(|(_, tbl)| !tbl.is_complete())
        {
            match tbl.add(num) {
                Result::Bingo(score) => {
                    if first_board.is_none() {
                        first_board = Some((i, num, score));
                    }
                    last_board.replace((i, num, score));
                }
                Result::Nope => (),
            }
        }
    }

    let (i, num, score) = first_board.expect("no winning board");
    println!("Bingo! First table {}", i);
    println!("Number is {} and score {}", num, score);
    println!("Answer: {}", num * score);

    let (i, num, score) = last_board.expect("no winning board");
    println!("Bingo! Last table {}", i);
    println!("Number is {} and score {}", num, score);
    println!("Answer: {}", num * score);
}
