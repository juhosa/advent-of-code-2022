#[derive(Debug, Clone)]
enum Direction {
    Top,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Tree {
    height: u32,
    visible: bool,
    directions: Vec<Direction>,
    row: u32,
    col: u32,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && self.row == other.row && self.col == other.col
    }
}

pub fn part1(input: &str) -> String {
    let mut data: Vec<Vec<Tree>> = input
        .lines()
        // .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .map(|line| {
            line.chars()
                .map(|c| Tree {
                    height: c.to_digit(10).unwrap(),
                    visible: false,
                    directions: Vec::new(),
                    row: 0,
                    col: 0,
                })
                .collect()
        })
        .collect();

    // println!("{:#?}", data);
    let mut count = 0;

    // outer trees are always visible
    count += (data.iter().len() * 2) as i32;
    count += ((data[0].len() - 2) * 2) as i32;

    // check rows
    let data_len = data.len();
    for (i, row) in data.iter_mut().enumerate() {
        if i == 0 || i == data_len - 1 {
            continue;
        }
        // println!("{:?}", row);
        for j in 1..data_len - 1 {
            // println!("{}", j);
            let mut trees_in_left: Vec<Tree> = row[..j].into();
            trees_in_left.sort_by(|a, b| a.height.cmp(&b.height));

            if trees_in_left.last().unwrap().height < row[j].height {
                // println!("from l: {:?} ({}) (row: {:?})", row[j], j, row);
                row[j].visible = true;
                row[j].directions.push(Direction::Left);
            }

            let mut trees_in_r: Vec<Tree> = row[j + 1..].into();
            trees_in_r.sort_by(|a, b| a.height.cmp(&b.height));

            if trees_in_r.last().unwrap().height < row[j].height {
                // println!("from r: {:?} ({}) (row: {:?})", row[j], j, row);
                row[j].visible = true;
                row[j].directions.push(Direction::Right);
            }
        }
    }

    // check columns
    let col_count = data.last().unwrap().len();
    // this transpose is from https://stackoverflow.com/a/64499219
    // let mut cols: Vec<Vec<Tree>> = (0..data[0].len())
    data = (0..data[0].len())
        .map(|i| {
            data.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<Tree>>()
        })
        .collect();

    let data_len = data.len();
    for (i, row) in data.iter_mut().enumerate() {
        if i == 0 || i == data_len - 1 {
            continue;
        }
        // println!("{:?}", row);
        for j in 1..data_len - 1 {
            // println!("{}", j);
            let mut trees_in_left: Vec<Tree> = row[..j].into();
            trees_in_left.sort_by(|a, b| a.height.cmp(&b.height));

            if trees_in_left.last().unwrap().height < row[j].height {
                // println!("from l: {:?} ({}) (row: {:?})", row[j], j, row);
                row[j].visible = true;
                row[j].directions.push(Direction::Top);
            }

            let mut trees_in_r: Vec<Tree> = row[j + 1..].into();
            trees_in_r.sort_by(|a, b| a.height.cmp(&b.height));

            if trees_in_r.last().unwrap().height < row[j].height {
                // println!("from r: {:?} ({}) (row: {:?})", row[j], j, row);
                row[j].visible = true;
                row[j].directions.push(Direction::Down);
            }
        }
    }

    // for row in cols {
    //     for tree in row {
    //         print!("{} ", tree.height);
    //     }
    //     println!();
    // }

    for row in data.iter() {
        for tree in row.iter() {
            // println!("{:?}", tree);
            if tree.visible {
                count += 1;
            }
        }
    }

    count.to_string()
}

pub fn part2(input: &str) -> String {
    let mut data: Vec<Tree> = Vec::new();

    let mut max_row = 0;
    let mut max_col = 0;
    for (row_index, row) in input.lines().enumerate() {
        max_row = row_index;
        // println!("{} {}", row_index, row);
        for (col_index, c) in row.chars().enumerate() {
            max_col = col_index;
            let t = Tree {
                visible: false,
                row: row_index as u32,
                col: col_index as u32,
                directions: Vec::new(),
                height: c.to_digit(10).unwrap(),
            };
            data.push(t);
        }
    }
    println!("max_row {}, max_col {}", max_row, max_col);

    // no need to check trees at the edges (because multiply by 0)
    for tree in data.iter() {
        if tree.row == 0
            || tree.col == 0
            || tree.row == max_row as u32
            || tree.col == max_col as u32
        {
            continue;
        }
        println!("checking {:?}", tree);
        let trees_in_row: Vec<&Tree> = data
            .iter()
            .filter(|t| t.row == tree.row && t != &tree)
            .collect();
        for tr in trees_in_row {
            println!("{:?}", tr);
        }
        break;
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_test_1() {
        let answer = part1(INPUT);
        assert_eq!(answer, "21")
    }

    #[test]
    fn part2_test_1() {
        let answer = part2(INPUT);
        assert_eq!(answer, "8")
    }
}
