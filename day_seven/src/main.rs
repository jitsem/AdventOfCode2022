//Credit to chris biscardi
use std::collections::BTreeMap;

const COMMANDS: &str = include_str!("../data/input.txt");

#[derive(Debug)]
enum Op<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Node<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}
#[derive(Debug)]
enum Node<'a> {
    File { name: &'a str, size: u32 },
    Folder(&'a str),
}

fn main() {
    let commands = parse_commands(COMMANDS);

    let (_, sizes) = commands.iter().fold(
        (vec![], BTreeMap::new()),
        calculate_sizes,
    );

    let res_1 = sizes
        .iter()
        .filter(|(_, &size)| size <= 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string();

    println!("part 1: {}", res_1);

    let (_, sizes) = commands.iter().fold(
        (vec![], BTreeMap::new()),
        calculate_sizes,
    );

    let free_space = 70000000 - sizes.get(&vec![""]).unwrap();
    let needed_space = 30000000 - free_space;
    println!("needed size to remove {}", needed_space);
    let mut candidates:Vec<&u32> = sizes
        .iter()
        .filter(|(_, &size)| size >= needed_space)
        .map(|(_, size)| size).collect();
    candidates.sort();
    let res_2 = candidates.first().unwrap();

    println!("part 2: {}", res_2)
}

fn parse_commands(input: &str) -> Vec<Op> {
    let lines: Vec<&str> = input.lines().collect();
    let mut commands = Vec::new();
    for i in 0..lines.len() {
        let line = lines[i];
        if !line.starts_with('$') {
            continue;
        }
        if line.starts_with("$ cd") {
            let command = match line.split_whitespace().nth(2) {
                Some(s) if s == "/" => Op::Cd(Cd::Root),
                Some(s) if s == ".." => Op::Cd(Cd::Up),
                Some(s) => Op::Cd(Cd::Down(s)),
                None => panic!("Unexpected cd command")
            };
            commands.push(command)
        } else if line.starts_with("$ ls") {
            let mut next = i + 1;

            let mut files = Vec::new() ;
            while next < lines.len() && !lines[next].starts_with('$') {
                let mut split = lines[next].split_whitespace();
                let node = match split.next() {
                    Some(val) if val == "dir" => Node::Folder(split.next().unwrap()),
                    Some(val) => Node::File {
                        name: split.next().unwrap(),
                        size: val.parse().unwrap(),
                    },
                    None => panic!("Unexpected ls command"),
                };
                files.push(node);
                next+=1;
            }
            commands.push(Op::Ls(files));
        }
    }
    commands
}

fn calculate_sizes<'a>(
        (mut context, mut sizes): (
                Vec<&'a str>,
        BTreeMap<Vec<&'a str>, u32>,
        ),
        command: &'a Op,
        ) -> (
                Vec<&'a str>,
BTreeMap<Vec<&'a str>, u32>,
) {
    match command {
        Op::Cd(Cd::Root) => {
            context.push("");
        }
        Op::Cd(Cd::Up) => {
            context.pop();
        }
        Op::Cd(Cd::Down(name)) => {
            context.push(name);
        }
        Op::Ls(files) => {
            let sum = files
            .iter()
            .filter_map(|file| {
                if let Node::File { size, .. } = file {
                    Some(size)
                } else {
                    None
                }
            })
            .sum::<u32>();

            for i in 0..context.len() {
                sizes
                .entry(context[0..=i].to_vec())
                .and_modify(|v| *v += sum)
                .or_insert(sum);
            }
        }
    };
    (context, sizes)
}
