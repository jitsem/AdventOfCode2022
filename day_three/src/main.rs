const RUGSACKS: &str = include_str!("../data/input.txt");

fn main() {
    let mut total_prio = 0;
    for (nr, line) in RUGSACKS.lines().enumerate() {
        if line.len() % 2 != 0 {
            panic!("Rugsack did not contain even nr of items")
        }
        println!("Rugsack {}: {}", nr, line);

        let (first, second) = line.split_at(line.len() / 2);
        for char in first.chars() {
            if second.contains(char) {
                println!("Duplicate: {}", char);
                if !char.is_alphabetic() {
                    panic!("Unexpected char in rugsack")
                }
                let prio: u32 = match char.is_lowercase() {
                    true => u32::from(char) - 96,
                    false => u32::from(char) - 38,
                };

                println!("Prio: {}", prio);
                total_prio += prio;
                break;
            }
        }
    }
    println!("Total Prio: {}", total_prio);

}
