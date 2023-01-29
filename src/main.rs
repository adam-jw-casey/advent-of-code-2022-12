use std::env;
use std::fs;
use advent_of_code_2022_12::shortest_path;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read {file_path}");

    println!("The shortest path is {} steps long", shortest_path(&contents));
}
