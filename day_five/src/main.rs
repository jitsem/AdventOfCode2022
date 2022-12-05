const SCHEDULE: &str = include_str!("../data/input.txt");
const NR_OF_STACKS: usize = 9;

#[derive(Debug)]
struct Crate {
    id: char,
}
impl Crate {
    fn new(input: &str) -> Self {
        let cs: Vec<char> = input.chars().collect();
        if cs.len() != 3 || cs[0] != '[' || cs[2] != ']' {
            panic!("Wrong input crate {}", input);
        }
        Crate { id: cs[1] }
    }
}

struct Move {
    amount: usize,
    source: usize,
    destination: usize,
}

impl Move {
    fn new(input: &str) -> Self {
        let instr: Vec<&str> = input.split_whitespace().collect();
        if instr.len() != 6 {
            panic!("Wrong input format")
        }
        if instr[0] != "move" {
            panic!("Wrong input format")
        }
        if instr[2] != "from" {
            panic!("Wrong input format")
        }
        if instr[4] != "to" {
            panic!("Wrong input format")
        }
        Move {
            amount: instr[1].parse().unwrap(),
            source: instr[3].parse().unwrap(),
            destination: instr[5].parse().unwrap(),
        }
    }
}
fn main() {
    let mut arr: [Vec<Crate>; NR_OF_STACKS] = Default::default();
    let split = SCHEDULE.split_once("\n\n").unwrap();
    for line in split.0.lines().rev().skip(1) {
        for (n, a) in arr.iter_mut().enumerate() {
            let token: &str = &line.chars().skip(4 * n).take(3).collect::<String>();
            if !token.trim().is_empty() {
                a.push(Crate::new(token))
            }
        }
    }
    for line in split.1.lines() {
        let next_move = Move::new(line);
        for _ in 0..next_move.amount {
            if let Some(moving) = arr[next_move.source - 1].pop() {
                arr[next_move.destination - 1].push(moving);
            }
        }
    }

    for a in arr.iter() {
        print!("{}", a.last().unwrap().id);
    }
    println!()
}
