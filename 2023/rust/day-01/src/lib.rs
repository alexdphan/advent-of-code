// Define a public function that takes a string slice as input and returns a String
pub fn process_part1(input: &str) -> String {
    // Declare a variable 'result' and assign it the value of 'input'
    let result = input
        // Split the input string into substrings at each occurrence of two consecutive newline characters
        .split("\n\n")
        // For each substring (referred to as 'elf_load'), perform the following operations
        // elf just means entry list of numbers
        .map(|elf_load| {
            // Split 'elf_load' (entry list of numbers) into substrings at each occurrence of a newline character
            // ex:
            // 1000
            // 2000
            // 3000
            elf_load
                .lines() // you can also just write .split("\n")
                // For each substring (referred to as 'item'), parse it as a u32 integer (using rust turbofish) and unwrap the Result (Ok or Error which should crash)
                .map(|item| item.parse::<u32>().unwrap())
                // Sum all the parsed integers
                .sum::<u32>()
        })
        // Find the maximum sum
        .max()
        // Unwrap the Result
        .unwrap();
    // Convert 'result' to a string
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        // getting the summed loads for each of the elfs which we collect into a vec
        .collect::<Vec<_>>();
    // we make the variable mutable to sort it high to low
    // Sort the vector 'result' in descending order
    // Comparing each pair of items 'a' and 'b' in the vector and ordering them based on the result of 'b.cmp(a)'
        // asking is b >= or < a instead of is a >= or < b, giving us reverse sorting of our vec
    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
}

// Attribute macro on top of module test
// The whole module (mod tests) will compile into a binary if we are running our test (cargo test)
#[cfg(test)]
mod tests {
    // This module contains tests for the code in this file.
    // This gives us access to the parent module, in this case it's the whole file, since tests is a submodule of the current file that we're working in
    use super::*;

    // conventional all caps for const
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    // test macro let's us know the regular function is a test, it lets us know if it works
    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000")
    }

    #[test]
    fn it2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000")
    }
}
