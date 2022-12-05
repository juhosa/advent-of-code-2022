pub fn part1(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut stack_count = 0;
    // loop in reverse to get the stack count first
    // and to fill the stacks in correct order (bottom up)
    for line in parts[0].lines().rev() {
        // println!("{line}");
        if stack_count == 0 {
            stack_count = line.chars().filter(|c| c.is_numeric()).count();
            // println!("stack_count: {}", stack_count);
            // create stack_count amount of empty stacks
            for _ in 0..stack_count {
                stacks.push(Vec::new());
            }
            continue;
        }

        let mut stack_index = 0;
        for k in line.chars().collect::<Vec<char>>().chunks(4) {
            // if start with space, increment index and continut
            // otherwise build stack
            // println!("k: {:#?}", k);
            if k[0] == ' ' {
                stack_index += 1;
                continue;
            }
            // index 1 holds the item
            stacks[stack_index].push(k[1]);
            stack_index += 1;
        }
    }

    // the moves
    for line in parts[1].lines() {
        // println!("{line}");
        let sections = line.split(' ').collect::<Vec<&str>>();
        // 1 = amount, 3 = origin, 5 = destination
        let amount: i32 = sections[1].parse().unwrap();
        let origin: i32 = sections[3].parse().unwrap();
        let dest: i32 = sections[5].parse().unwrap();
        // println!("{} {} {}", amount, origin, dest);
        for _ in 0..amount {
            let t = stacks[(origin - 1) as usize].pop().unwrap();
            stacks[(dest - 1) as usize].push(t);
        }
    }

    // for stack in &stacks {
    //     println!("{:?}", stack);
    // }

    let result = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();

    result
}

pub fn part2(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut stack_count = 0;
    // loop in reverse to get the stack count first
    // and to fill the stacks in correct order (bottom up)
    for line in parts[0].lines().rev() {
        // println!("{line}");
        if stack_count == 0 {
            stack_count = line.chars().filter(|c| c.is_numeric()).count();
            // println!("stack_count: {}", stack_count);
            // create stack_count amount of empty stacks
            for _ in 0..stack_count {
                stacks.push(Vec::new());
            }
            continue;
        }

        let mut stack_index = 0;
        for k in line.chars().collect::<Vec<char>>().chunks(4) {
            // if start with space, increment index and continut
            // otherwise build stack
            // println!("k: {:#?}", k);
            if k[0] == ' ' {
                stack_index += 1;
                continue;
            }
            // index 1 holds the item
            stacks[stack_index].push(k[1]);
            stack_index += 1;
        }
    }

    // the moves
    for line in parts[1].lines() {
        // println!("{line}");
        let sections = line.split(' ').collect::<Vec<&str>>();
        // 1 = amount, 3 = origin, 5 = destination
        let amount: i32 = sections[1].parse().unwrap();
        let origin: i32 = sections[3].parse().unwrap();
        let dest: i32 = sections[5].parse().unwrap();

        let l = stacks[(origin - 1) as usize].len();

        // get the items to move
        let t: Vec<char> = stacks[(origin - 1) as usize]
            .iter()
            .rev()
            .take(amount as usize)
            .rev()
            .copied() // this is required to move the ownership (otherwise would be &char)
            .collect::<Vec<char>>();

        // insert into the new stack
        stacks[(dest - 1) as usize].append(&mut t.into_iter().collect::<Vec<char>>());

        // remove items from the old
        stacks[(origin - 1) as usize].drain(l - amount as usize..);
    }

    let result = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_test() {
        let answer = part1(TEST_INPUT);
        assert_eq!(answer, "CMZ")
    }

    #[test]
    fn part2_test() {
        let answer = part2(TEST_INPUT);
        assert_eq!(answer, "MCD")
    }
}
