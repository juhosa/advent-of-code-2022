use std::fs;

fn main() {
    println!("Day 02");

    // let filename = "test-input.txt".to_string();
    let filename = "input.txt".to_string();

    let contents = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(e) => panic!("error reading input file: {e}"),
    };

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    let mut total_score = 0;
    for line in contents.lines() {
        let mut score = 0;
        let parts: Vec<&str> = line.split(' ').collect();
        // dbg!(parts);
        // Rock = A and X
        // Paper = B and Y
        // Scissors = C and Z
        match parts[1] {
            // rock
            "X" => {
                score += 1;
                match parts[0] {
                    "A" => score += 3,
                    "B" => score += 0,
                    "C" => score += 6,
                    _ => unreachable!("heh"),
                }
            }
            // paper
            "Y" => {
                score += 2;
                match parts[0] {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => score += 0,
                    _ => unreachable!("heh"),
                }
            }
            // scissors
            "Z" => {
                score += 3;
                match parts[0] {
                    "A" => score += 0,
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => unreachable!("heh"),
                }
            }
            _ => {
                unreachable!("oh boi")
            }
        }

        // println!("round score: {}", score);
        total_score += score;
    }

    println!("total score: {}", total_score);
}

fn part2(contents: &str) {
    let mut total_score = 0;
    for line in contents.lines() {
        let mut score = 0;
        let parts: Vec<&str> = line.split(' ').collect();
        // dbg!(parts);
        // Rock = A 1
        // Paper = B 2
        // Scissors = C 3
        // X = lose 0
        // Y = draw 3
        // Z = win 6
        match parts[0] {
            "A" => match parts[1] {
                "X" => score += 3,
                "Y" => score += 4,
                "Z" => score += 8,
                _ => unreachable!("heh"),
            },
            "B" => match parts[1] {
                "X" => score += 1,
                "Y" => score += 5,
                "Z" => score += 9,
                _ => unreachable!("heh"),
            },
            "C" => match parts[1] {
                "X" => score += 2,
                "Y" => score += 6,
                "Z" => score += 7,
                _ => unreachable!("heh"),
            },
            _ => {
                unreachable!("oh boi")
            }
        }

        // println!("round score: {}", score);
        total_score += score;
    }

    println!("total score: {}", total_score);
}
