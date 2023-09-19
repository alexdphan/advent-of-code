use day_07::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let result = process_part1(&file);
    println!("Result: {}", result);
    println!("Part 1 done");
}
