const ASSIGNMENTS: &str = include_str!("../data/input.txt");

struct Range {
    lower: u32,
    upper: u32,
}

fn main() {
    let mut amount_fully_contained = 0;
    let mut amount_overlapping = 0;
    for line in ASSIGNMENTS.lines() {
        let (one, two) = create_range_pair(line);
        if (one.lower >= two.lower && one.upper <= two.upper  ) || (one.lower <= two.lower && one.upper >= two.upper)
        {
            amount_fully_contained +=1;
        }
        if !(one.upper < two.lower || one.lower > two.upper){
            amount_overlapping +=1;
        }
    }
    println!("Total amount of fully contained assingment: {}", amount_fully_contained);
    println!("Total amount of overlapping assingment: {}", amount_overlapping);
}

fn create_range_pair(input: &str) -> (Range, Range) {
    if !input.contains(',') {
        panic!("Wrong pair format");
    }

    let pairs: Vec<&str> = input.split(',').collect();

    if !&pairs.len() == 2 {
        panic!("Wrong amount of pairs");
    }
    if !&pairs[0].contains('-') || !&pairs[1].contains('-') {
        panic!("Pairs were invalid")
    }

    let range1: Vec<&str> = pairs[0].split('-').collect();
    let range2: Vec<&str> = pairs[1].split('-').collect();
    if !&range1.len() == 2 || !&range2.len() == 2 {
        panic!("Ranges were not correct");
    }
    (
        Range {
            lower: range1[0].parse().unwrap(),
            upper: range1[1].parse().unwrap(),
        },
        Range {
            lower: range2[0].parse().unwrap(),
            upper: range2[1].parse().unwrap(),
        },
    )
}
