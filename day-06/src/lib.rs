use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> String {
    let mut prev_four: VecDeque<char> = VecDeque::new();

    let mut result = 0;

    for (count, c) in input.chars().enumerate() {
        // handle the first 4 chars
        if prev_four.len() < 4 {
            prev_four.push_back(c);
            continue;
        }

        let asd: HashSet<char> = prev_four.clone().into_iter().collect();
        if asd.iter().len() == 4 {
            result = count;
            break;
        }
        prev_four.pop_front().unwrap();
        prev_four.push_back(c);
    }

    result.to_string()
}
pub fn part2(input: &str) -> String {
    let mut previous: VecDeque<char> = VecDeque::new();

    let mut result = 0;

    for (count, c) in input.chars().enumerate() {
        // handle the first 14 chars
        if previous.len() < 14 {
            previous.push_back(c);
            continue;
        }

        // check uniqueness
        let asd: HashSet<char> = previous.clone().into_iter().collect();
        if asd.iter().len() == 14 {
            result = count;
            break;
        }
        previous.pop_front().unwrap();
        previous.push_back(c);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_test_1() {
        let answer = part1(INPUT1);
        assert_eq!(answer, "7")
    }

    #[test]
    fn part1_test_2() {
        let answer = part1(INPUT2);
        assert_eq!(answer, "5")
    }

    #[test]
    fn part1_test_3() {
        let answer = part1(INPUT3);
        assert_eq!(answer, "6")
    }

    #[test]
    fn part1_test_4() {
        let answer = part1(INPUT4);
        assert_eq!(answer, "10")
    }

    #[test]
    fn part1_test_5() {
        let answer = part1(INPUT5);
        assert_eq!(answer, "11")
    }

    #[test]
    fn part2_test_1() {
        let answer = part2(INPUT1);
        assert_eq!(answer, "19")
    }

    #[test]
    fn part2_test_2() {
        let answer = part2(INPUT2);
        assert_eq!(answer, "23")
    }

    #[test]
    fn part2_test_3() {
        let answer = part2(INPUT3);
        assert_eq!(answer, "23")
    }

    #[test]
    fn part2_test_4() {
        let answer = part2(INPUT4);
        assert_eq!(answer, "29")
    }

    #[test]
    fn part2_test_5() {
        let answer = part2(INPUT5);
        assert_eq!(answer, "26")
    }
}
