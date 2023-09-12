use day_02::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let result = process_part2(&file);
    println!("Result: {}", result);
    println!("Part 2 done");
}
