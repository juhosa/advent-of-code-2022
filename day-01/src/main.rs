use std::fs;

fn main() {
    println!("Day 01");

    let contents = match fs::read_to_string("input.txt") {
        Ok(data) => data,
        Err(e) => panic!("error! {}", e),
    };

    let mut elf_calories = 0;
    let mut calories = vec![];

    for line in contents.lines() {
        // println!("{line}");
        if line.is_empty() {
            // println!("Total calories {}", elf_calories);
            calories.push(elf_calories);
            elf_calories = 0;
            continue;
            // break;
        }
        let num: i32 = line.parse().expect("failed to parse");
        // println!("{}", num);
        elf_calories += num;
    }

    calories.sort();
    // let max = calories.iter().max_by(|x, y| x.cmp(y)).unwrap();
    let max = calories.last().unwrap();
    println!("Part 1:");
    println!("max {}", max);

    println!("Part 2:");

    let t: i32 = calories[calories.len() - 3..calories.len()].iter().sum();
    println!("first 3 total: {}", t);
}
