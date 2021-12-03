use advent::day2::{
    Position,
    Aim,
    MoveCommand,
};

static DAY2_INPUT: &str = include_str!("../input/day2.txt");


fn main() {
    let mut pos = Position::default();

    let mut aimed_pos = Aim::default();

    let commands = DAY2_INPUT
        .lines()
        .map(|line| line.parse::<MoveCommand>().expect("Invalid command"));

    for cmd in commands {
        pos.apply(&cmd);
        aimed_pos.apply(&cmd);
    }
    println!("Our position is {:?}: {}", pos, pos.product());

    println!("Okay, now our position is {:?}: {}",
        aimed_pos, aimed_pos.position.product());
}
