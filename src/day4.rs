use std::str::FromStr;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

const N: usize = 5;

pub fn main(input: &str) -> (usize, usize) {

    let mut it = input.split_terminator("\n\n");
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
    let score1 = num * score;
    println!("Answer: {}", num * score);

    let (i, num, score) = last_board.expect("no winning board");
    println!("Bingo! Last table {}", i);
    println!("Number is {} and score {}", num, score);
    let score2 = num * score;
    (score1, score2)
}



#[derive(Debug)]
pub enum Result {
    Bingo(usize),
    Nope,
}

#[derive(Debug)]
enum Mark {
    Unchecked,
    Checked,
}

#[derive(Debug)]
struct Cell {
    row: u8,
    col: u8,
}

#[derive(Debug)]
pub struct Table {
    table: HashMap<usize, (Mark, Cell)>,
    rows_match_count: [u8; N],
    cols_match_count: [u8; N],
}

impl Table {
    pub fn add(&mut self, num: usize) -> Result {
        let val = self.table
            .entry(num)
            .and_modify(|v| {
                v.0 = Mark::Checked;
            });
        match val {
            Entry::Occupied(e) => {
                let cell = &e.get().1;
                let rows = self.rows_match_count[cell.row as usize] + 1;
                let cols = self.cols_match_count[cell.col as usize] + 1;
                self.rows_match_count[cell.row as usize] = rows;
                self.cols_match_count[cell.col as usize] = cols;
                if rows == 5 || cols == 5 {
                    Result::Bingo(self.score())
                } else {
                    Result::Nope
                }
            }
            Entry::Vacant(_) => Result::Nope,
        }
    }

    pub fn score(&self) -> usize {
        self.table
            .iter()
            .filter(|(_, (m, _))| matches!(m, Mark::Unchecked))
            .map(|(k, _)| *k)
            .sum()
    }

    pub fn is_complete(&self) -> bool {
        self.rows_match_count
            .iter()
            .chain(&self.cols_match_count)
            .any(|x| *x == N as u8)
    }
}

impl Default for Table {
    fn default() -> Self {
        Table {
            table: HashMap::with_capacity(N*N),
            rows_match_count: [0; N],
            cols_match_count: [0; N],
        }
    }
}

impl FromStr for Table {
    type Err = String;

    fn from_str(val: &str) -> std::result::Result<Self, Self::Err> {
        let mut result = Table::default();
        for (i, s) in val
            .split(|c| c == '\n' || c == ' ')
            .filter(|sub| !sub.is_empty())
            .enumerate()
            .take(N*N)
        {
            let val = s.parse().map_err(|e| format!("{}", e))?;
            result.table.insert(val, (Mark::Unchecked, Cell::from_index(i)));
        }
        Ok(result)
    }
}

impl Cell {
    fn from_index(i: usize) -> Self {
        Cell {
            row: (i / N) as u8,
            col: (i % N) as u8,
        }
    }
}
