const ELVES: &str = include_str!("../data/input.txt");

fn main() {
    println!("Hello, world!");
    let mut elves = Vec::new();
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
    for (pos,elf) in elves.iter().enumerate()
    {
        println!("Found elf nr {} with {} calories", pos, elf)
    }
}
