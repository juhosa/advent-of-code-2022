use std::fs;

fn main() {
    println!("Day 01");

    let contents = match fs::read_to_string("input.txt") {
        Ok(data) => data,
        Err(e) => panic!("error! {}", e),
    };

    let mut calories: Vec<i32> = contents
        .split("\n\n")
        .map(|group| group.lines().map(|num| num.parse::<i32>().unwrap()).sum())
        .collect();

    // sort in reverse (highest first)
    calories.sort_by(|a, b| b.cmp(a));
    let max = calories.first().unwrap();

    println!("Part 1:");
    println!("max {}", max);

    println!("Part 2:");

    let k: i32 = calories.iter().take(3).sum();
    println!("first 3 total: {}", k);
}
