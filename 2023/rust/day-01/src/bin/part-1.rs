// functions that are pulled in can run as long as they are in bin (with the "use" keyword)

// Importing the function process_part1 from the module day_01
use day_01::process_part1;
// Importing the module fs from the standard library
use std::fs;

// Main function where the program starts execution
fn main() {
    // Reading the file input.txt and storing the content in the variable file
    let file = fs::read_to_string("./input.txt").unwrap();
    // Calling the function process_part1 with file as parameter and printing the result
    // Execute the function process_part1 with file as parameter and store the result
    let result = process_part1(&file);
    // Print the result
    println!("Result: {}", result);
    // Print completion message
    println!("Part 1 done");
}
