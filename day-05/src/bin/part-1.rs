use std::fs;

use day_05::part1;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    println!("part 1 answer: {}", part1(&contents));
}
