const STRATEGY: &str = include_str!("../data/input.txt");

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scisor,
}

fn main() {
    let mut total_score = 0;
    for line in STRATEGY.lines() {
        let opp = match line.chars().next().unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scisor,
            _ => panic!("Invalid move"),
        };

        let own = match line.chars().nth(2).unwrap() {
            'X' => (Move::Rock, 1),
            'Y' => (Move::Paper, 2),
            'Z' => (Move::Scisor, 3),
            _ => panic!("Invalid move"),
        };

        let score = match opp {
            Move::Rock => match own.0 {
                Move::Rock => 3,
                Move::Paper => 6,
                Move::Scisor => 0,
            },
            Move::Paper => match own.0 {
                Move::Rock => 0,
                Move::Paper => 3,
                Move::Scisor => 6,
            },
            Move::Scisor => match own.0 {
                Move::Rock => 6,
                Move::Paper => 0,
                Move::Scisor => 3,
            },
        };
        total_score = total_score + score + own.1;
    }

    println!("Total score 1 : {}", total_score);

    let mut total_score_2 = 0;
    for line in STRATEGY.lines() {
        let opp = match line.chars().next().unwrap() {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scisor,
            _ => panic!("Invalid move"),
        };

        let score = match line.chars().nth(2).unwrap() {
            'X' => match opp {
                Move::Rock => 3,
                Move::Paper => 1,
                Move::Scisor => 2,
            },
            'Y' => match opp {
                Move::Rock => 1 + 3,
                Move::Paper => 2 + 3,
                Move::Scisor => 3 + 3,
            },
            'Z' => match opp {
                Move::Rock => 2 + 6,
                Move::Paper => 3 + 6,
                Move::Scisor => 1 + 6,
            },
            _ => panic!("Invalid move"),
        };

        total_score_2 += score;
    }
    println!("Total score 2 : {}", total_score_2);
}
