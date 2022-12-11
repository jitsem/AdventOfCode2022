const TREES: &str = include_str!("../data/input.txt");

#[derive(Clone, Debug)]
enum Tree {
    Unchecked { value: u32 },
    Checked { value: u32, seen: bool, score: u32 },
}
impl Tree {
    fn get_value(&self) -> u32 {
        match self {
            Self::Unchecked { value } => *value,
            Self::Checked { value, .. } => *value,
        }
    }
}
fn main() {
    println!("Hello, world!");
    let nr_of_vis = do_calculation_part_1(TREES);
    let max_scenic_score = do_calculation_part_2(TREES);
    println!("Numnber of visible trees: {}", nr_of_vis);
    println!("Max scenic score: {}", max_scenic_score);
}

fn do_calculation_part_1(input: &str) -> u32 {
    let trees = check_trees(input);
    let nr_of_vis = trees
        .iter()
        .flatten()
        .filter(|t| match t {
            Tree::Unchecked { .. } => false,
            Tree::Checked { seen, .. } => *seen,
        })
        .count();
    nr_of_vis as u32
}

fn do_calculation_part_2(input: &str) -> u32 {
    let trees = check_trees(input);
    let max_score = trees
        .iter()
        .flatten()
        .map(|t| match t {
            Tree::Unchecked { .. } => 0,
            Tree::Checked { score, .. } => *score,
        })
        .max();
    max_score.unwrap()
}
fn check_trees(input: &str) -> Vec<Vec<Tree>> {
    let tree_width = input.lines().next().unwrap().chars().count();
    let tree_len = input.lines().next().unwrap().len();
    let mut trees: Vec<Vec<Tree>> = vec![vec![Tree::Unchecked { value: 0 }; tree_width]; tree_len];

    for (line_nr, tree_line) in input.lines().enumerate() {
        for (tree_nr, tree) in tree_line.chars().enumerate() {
            trees[line_nr][tree_nr] = Tree::Unchecked {
                value: tree.to_digit(10).unwrap(),
            };
        }
    }
    for line_nr in 0..tree_len {
        for tree_nr in 0..tree_width {
            let tree = &trees[line_nr][tree_nr];
            let seen = check_visible(&trees, line_nr, tree_nr);
            let score = check_scenic_score(&trees, line_nr, tree_nr);

            trees[line_nr][tree_nr] = Tree::Checked {
                value: tree.get_value(),
                seen,
                score,
            };
        }
    }
    trees
}
fn check_visible(trees: &Vec<Vec<Tree>>, line_nr: usize, tree_nr: usize) -> bool {
    let width = trees[0].len();
    let len = trees.len();
    if tree_nr == 0 || line_nr == 0 || tree_nr == width - 1 || line_nr == len - 1 {
        return true;
    }
    let value = trees[line_nr][tree_nr].get_value();

    let mut up = line_nr - 1;
    loop {
        if trees[up][tree_nr].get_value() >= value {
            break;
        }
        if up == 0 {
            return true;
        }
        up -= 1;
    }
    let mut down = line_nr + 1;
    loop {
        if trees[down][tree_nr].get_value() >= value {
            break;
        }
        if down == len - 1 {
            return true;
        }
        down += 1;
    }

    let mut left = tree_nr - 1;
    loop {
        if trees[line_nr][left].get_value() >= value {
            break;
        }

        if left == 0 {
            return true;
        }
        left -= 1;
    }

    let mut right = tree_nr + 1;
    loop {
        if trees[line_nr][right].get_value() >= value {
            break;
        }
        if right == width - 1 {
            return true;
        }
        right += 1;
    }
    false
}

fn check_scenic_score(trees: &Vec<Vec<Tree>>, line_nr: usize, tree_nr: usize) -> u32 {
    let width = trees[0].len();
    let len = trees.len();
    let mut score_up = 0;
    let mut score_down = 0;
    let mut score_left = 0;
    let mut score_right = 0;

    let value = trees[line_nr][tree_nr].get_value();

    if line_nr != 0 {
        let mut up = line_nr - 1;
        loop {
            score_up += 1;
            if trees[up][tree_nr].get_value() >= value {
                break;
            }
            if up == 0 {
                break;
            }
            up -= 1;
        }
    }
    if line_nr != len - 1 {
        let mut down = line_nr + 1;
        loop {
            score_down += 1;
            if trees[down][tree_nr].get_value() >= value {
                break;
            }
            if down == len - 1 {
                break;
            }
            down += 1;
        }
    }
    if tree_nr != 0 {
        let mut left = tree_nr - 1;
        loop {
            score_left += 1;
            if trees[line_nr][left].get_value() >= value {
                break;
            }

            if left == 0 {
                break;
            }
            left -= 1;
        }
    }
    if tree_nr != width - 1 {
        let mut right = tree_nr + 1;
        loop {
            score_right += 1;
            if trees[line_nr][right].get_value() >= value {
                break;
            }
            if right == width - 1 {
                break;
            }
            right += 1;
        }
    }
    score_up * score_down * score_left * score_right
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
    fn part1() {
        assert_eq!(do_calculation_part_1(INPUT), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(do_calculation_part_2(INPUT), 8);
    }
}
