use std::str::FromStr;

static DAY2_INPUT: &str = include_str!("../input/day2.txt");


fn main() {
    let mut pos = Position::default();

    let commands = DAY2_INPUT
        .lines()
        .map(|line| line.parse::<MoveCommand>().expect("Invalid command"));

    for cmd in commands {
        pos = pos.apply(&cmd);
    }
    println!("Our position is {:?}: {}", pos, pos.horizontal * pos.depth);
}


#[derive(Debug)]
pub enum MoveCommand {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for MoveCommand {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let (cmd, unit) = val.split_once(' ')
            .ok_or_else(|| "Line without space".to_string())
            .and_then(|(cmd, unit)| {
                unit.parse::<usize>()
                    .map_err(|e| format!("{}", e))
                    .map(|unit| (cmd, unit))
            })?;
        let cmd = match cmd {
            "forward" => MoveCommand::Forward(unit),
            "down" => MoveCommand::Down(unit),
            "up" => MoveCommand::Up(unit),
            _ => return Err("Unexpected command".to_string()),
        };
        Ok(cmd)
    }
}

#[derive(Debug, Default)]
pub struct Position {
    pub horizontal: usize,
    pub depth: usize,
}


impl Position {
    pub fn apply(self, cmd: &MoveCommand) -> Self {
        use self::MoveCommand::*;

        let (horizontal, depth) = match cmd {
            Forward(x) => (self.horizontal + x, self.depth),
            Down(x) => (self.horizontal, self.depth + x),
            Up(x) => (self.horizontal, self.depth - x),
        };
        Position {
            horizontal,
            depth,
        }
    }
}
