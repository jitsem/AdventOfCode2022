use std::collections::HashSet;

const STREAM: &str = include_str!("../data/input.txt");
const BUFFER_SIZE: usize = 32;

fn main() {
    let res1 = find_marker(4);
    println!("Packet start @ {}", res1+1);
    let res2 = find_marker(14);
    println!("Message start @ {}", res2+1);
}

fn find_marker(nr_of_chars:usize) -> usize{
    if nr_of_chars > BUFFER_SIZE
            {
                panic!("Only 32 chars supported");
            }
    let mut queue: [char; BUFFER_SIZE] = Default::default();
    let mut index = 0;
    for (pos, c) in STREAM.chars().enumerate() {
        queue[index] = c;
        let unique_queue = queue
        .into_iter()
        .take(nr_of_chars)
        .collect::<HashSet<_>>()
        .into_iter();
        if pos > nr_of_chars - 2 && unique_queue.len() == nr_of_chars {
            println!("Found {:?} @ {} which is {}th pos", queue, pos, pos+1);
            return pos;
        }
        index += 1;
        if index == nr_of_chars {
            index = 0;
        }
    }
    panic!("Marker not found")
}
