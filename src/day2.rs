use std::str::FromStr;

pub fn main(input: &str) -> (Position, Aim) {
    let mut pos = Position::default();
    let mut aimed_pos = Aim::default();

    let commands = input
        .lines()
        .map(|line| line.parse::<MoveCommand>().expect("Invalid command"));

    for cmd in commands {
        pos.apply(&cmd);
        aimed_pos.apply(&cmd);
    }
    println!("Our position is: {:?}", pos);
    println!("Okay, now our position is: {:?}", aimed_pos);
    (pos, aimed_pos)
}

#[derive(Debug)]
pub enum MoveCommand {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for MoveCommand {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let (cmd, unit) = val.split_once(' ')
            .ok_or_else(|| "Line without space".to_string())
            .and_then(|(cmd, unit)| {
                unit.parse::<isize>()
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
    pub horizontal: isize,
    pub depth: isize,
}


impl Position {
    pub fn apply(&mut self, cmd: &MoveCommand) {
        use self::MoveCommand::*;

        match cmd {
            Forward(x) => self.horizontal += x,
            Down(x) => self.depth += x,
            Up(x) => self.depth -= x,
        };
    }

    pub fn product(&self) -> isize {
        self.horizontal * self.depth
    }
}

#[derive(Debug, Default)]
pub struct Aim {
    pub aim: isize,
    pub position: Position,
}

impl Aim {

    pub fn apply(&mut self, cmd: &MoveCommand) {
        use self::MoveCommand::*;

        match *cmd {
            Forward(x) => {
                self.position.apply(cmd);
                self.position.apply(&Down(self.aim * x))
            }
            Down(x) => {
                self.aim += x;
            }
            Up(x) => {
                self.aim -= x;
            }
        };
    }
}
