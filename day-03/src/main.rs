use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Day 03");

    // let filename = "test-input.txt".to_string();
    let filename = "input.txt".to_string();

    let contents = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(e) => panic!("error reading input file: {e}"),
    };

    // part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    let mut d = ('a'..='z').collect::<Vec<char>>();
    let mut d2 = ('A'..='Z').collect::<Vec<char>>();

    d.append(&mut d2);

    let mut total_score = 0;
    for line in contents.lines() {
        let len = line.len();
        let begin = &line[..len / 2];
        let end = &line[len / 2..];

        let hs1: HashSet<char> = HashSet::from_iter(begin.chars());
        let hs2: HashSet<char> = HashSet::from_iter(end.chars());

        let in_both: Vec<&char> = hs1.intersection(&hs2).into_iter().collect();

        let score: usize = in_both
            .iter()
            .map(|c| d.iter().position(|&d| d == **c).unwrap() + 1)
            .collect::<Vec<usize>>()
            .into_iter()
            .sum();

        total_score += score;
    }

    println!("total_score: {}", total_score);
}

fn part2(contents: &str) {
    let mut d = ('a'..='z').collect::<Vec<char>>();
    let mut d2 = ('A'..='Z').collect::<Vec<char>>();

    d.append(&mut d2);
    let mut total_score = 0;

    let lines = contents.lines().collect::<Vec<&str>>();
    for i in (0..lines.len()).step_by(3) {
        let mut sets: Vec<HashSet<char>> = Vec::new();
        let h1: HashSet<char> = HashSet::from_iter(lines[i].chars());
        let h2: HashSet<char> = HashSet::from_iter(lines[i + 1].chars());
        let h3: HashSet<char> = HashSet::from_iter(lines[i + 2].chars());
        sets.push(h1.clone());
        sets.push(h2.clone());
        sets.push(h3.clone());

        // this is from
        // https://github.com/rust-lang/rfcs/issues/2023#issuecomment-739483074
        let inter = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });

        let score: usize = inter
            .iter()
            .map(|c| d.iter().position(|&d| d == *c).unwrap() + 1)
            .collect::<Vec<usize>>()
            .into_iter()
            .sum();

        total_score += score;
    }
    println!("total_score: {}", total_score);
}
