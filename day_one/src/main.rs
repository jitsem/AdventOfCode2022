const ELVES: &str = include_str!("../data/input.txt");

fn main() {
    let mut elves = vec![0];
    elves.push(0);
    for line in ELVES.lines() {
        if line.trim().is_empty() {
            elves.push(0)
        } else {
            let x = elves.pop().expect("No elements ion array");
            let calories: u32 = line.parse().unwrap();
            elves.push(x + calories);
        }
        println!("Found {}", line);
    }

    elves.sort();
    for (pos, elf) in elves.iter().enumerate() {
        println!("Found elf nr {} with {} calories", pos, elf)
    }

    let nr_of_elves = elves.len() - 1;
    println!("Top elf: {}", elves[nr_of_elves]);
    let top_3 = elves[nr_of_elves] + elves[nr_of_elves - 1] + elves[nr_of_elves - 2];
    println!("Top 3 elfes: {}", top_3);
}
