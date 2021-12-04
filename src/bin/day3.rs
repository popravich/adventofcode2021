static DATA: &str = include_str!("../input/day3.txt");

use advent::day3;

fn main() {
    let metrics = day3::main(DATA);

    println!("Gamma rate: {}", metrics.gamma_rate);
    println!("Epsilon rate: {}", metrics.epsilon_rate);
    println!("Power consumption: {}", metrics.power_consumtion());

    println!("Oxygen generator: {}", metrics.oxygen_generator);
    println!("CO2 rating: {}", metrics.co2_rating);
    println!("Life support rating: {}", metrics.life_support_rating());
}
