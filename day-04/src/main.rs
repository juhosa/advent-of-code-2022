use std::fs;

fn main() {
    println!("Day 04");

    // let filename = "test-input.txt".to_string();
    let filename = "input.txt".to_string();

    let contents = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(e) => panic!("error reading input file: {e}"),
    };

    let mut result = part1(&contents);
    println!("part 1 result: {}", result);

    result = part2(&contents);
    println!("part 2 result: {}", result);
}

fn part1(contents: &str) -> String {
    let mut count = 0;

    for line in contents.lines() {
        // println!("line: {}", line);

        let mut pairs: Vec<Vec<i32>> = Vec::new();

        for pair in line.split(',') {
            // println!("{pair}");
            let k = pair
                .split('-')
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            // println!("{:?}", k);
            pairs.push(k);
        }

        // println!("{:?}", pairs);
        if pairs[0][0] <= pairs[1][0] && pairs[0][1] >= pairs[1][1]
            || pairs[1][0] <= pairs[0][0] && pairs[1][1] >= pairs[0][1]
        {
            // println!("k");
            // println!("{:?}", pairs);
            count += 1;
        }
    }

    count.to_string()
}

fn part2(contents: &str) -> String {
    let mut count = 0;

    for line in contents.lines() {
        // println!("line: {}", line);

        let mut pairs: Vec<Vec<i32>> = Vec::new();

        for pair in line.split(',') {
            // println!("{pair}");
            let k = pair
                .split('-')
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            // println!("{:?}", k);
            pairs.push(k);
        }

        // println!("{:?}", pairs);
        let p1_values = (pairs[0][0]..=pairs[0][1]).collect::<Vec<i32>>();
        let p2_values = (pairs[1][0]..=pairs[1][1]).collect::<Vec<i32>>();

        // println!("line: {}", line);
        // println!("{:?}", p1_values);
        // println!("{:?}", p2_values);

        for value in p1_values {
            if p2_values.contains(&value) {
                // println!("overlapping value: {value}");
                count += 1;
                break;
            }
        }
    }

    count.to_string()
}
