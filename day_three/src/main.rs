const RUGSACKS: &str = include_str!("../data/input.txt");

fn main() {
    let mut total_prio = 0;
    for line in RUGSACKS.lines() {
        if line.len() % 2 != 0 {
            panic!("Rugsack did not contain even nr of items")
        }

        let (first, second) = line.split_at(line.len() / 2);
        for char in first.chars() {
            if second.contains(char) {
                if !char.is_alphabetic() {
                    panic!("Unexpected char in rugsack")
                }
                let prio: u32 = match char.is_lowercase() {
                    true => u32::from(char) - 96,
                    false => u32::from(char) - 38,
                };

                total_prio += prio;
                break;
            }
        }
    }
    println!("Total Prio part 1: {}", total_prio);

    let mut total_prio_2 = 0;

    for ((one, two), three) in RUGSACKS
        .lines()
        .step_by(3)
        .zip(RUGSACKS.lines().skip(1).step_by(3))
        .zip(RUGSACKS.lines().skip(2).step_by(3))
    {
        for char in one.chars() {
            if two.contains(char) && three.contains(char) {
                if !char.is_alphabetic() {
                    panic!("Unexpected char in rugsack")
                }
                let prio: u32 = match char.is_lowercase() {
                    true => u32::from(char) - 96,
                    false => u32::from(char) - 38,
                };

                total_prio_2 += prio;
                break;
            }
        }
    }
    println!("Total Prio part 2: {}", total_prio_2);
}
