use advent::day2;

static DATA: &str = include_str!("../input/day2.txt");

fn main() {
    let (pos, aimed_pos) = day2::main(DATA);
    println!("Our position is {:?}: {}", pos, pos.product());
    println!("Okay, now our position is {:?}: {}",
        aimed_pos, aimed_pos.position.product());
}
