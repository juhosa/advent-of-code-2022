use std::fs;

use day_05::part2;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    println!("part 2 answer: {}", part2(&contents));
}
