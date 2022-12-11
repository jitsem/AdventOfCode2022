const TREES: &str = include_str!("../data/input.txt");

#[derive(Clone, Debug)]
enum Tree {
    Unchecked { value: u32 },
    Checked { value: u32, seen: bool },
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
    println!("Numnber of visible trees: {}", nr_of_vis);
}

fn do_calculation_part_1(input: &str) -> u32 {
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

            trees[line_nr][tree_nr] = Tree::Checked {
                value: tree.get_value(),
                seen,
            };
        }
    }
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
}
