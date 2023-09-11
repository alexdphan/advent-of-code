// explain this line of code in one comment
use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    // because we use indexes to define the scores, they are defined as usize, which is an unsigned integer
    // usize is the default integer type in Rust, which is about 64 bits on a 64-bit architecture and 32 bits on a 32-bit architecture. It's platform-dependent.
    let letter_scores = ('a'..='z')
        // chain: https://doc.rust-lang.org/std/iter/struct.Chain.html
        // Chainning two ranges together
        // we use .chain to chain the lowercase letters and uppercase letters together
        // this is because we want to iterate over both the lowercase letters and uppercase letters
        .chain('A'..='Z')
        // into_iter converts the collection (the chain) into an iterator
        // https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
        // ex: [1, 2, 3] is an iterator
        // into_iter converts the collection into an iterator
        .into_iter()
        // .enumerate() returns an iterator of tuples where the first element is the index and the second element is the value
        // for example, "hello", we would enumerate for each character
        .enumerate()
        // mapped over this and translate this into a tuple
        // This line of code maps each tuple (idx, c) to a new tuple (c, idx + 1)
        // here, it is mapping each tuple (idx, c) to a new tuple (c, idx + 1)
        // ex: 'a' would be mapped to (a, 1)
        .map(|(idx, c)| (c, idx + 1))
        // Collects the key-value pairs from the iterator into a HashMap
        // if you collect a tuple, you can collect a HashMap where the key is on the left and value is on the right
        .collect::<HashMap<char, usize>>();
    // A HashMap is a collection of key-value pairs where each key is unique.
    // It provides constant-time complexity for insertion, deletion, and retrieval operations.
    // Example:
    // Let's say we want to count the frequency of characters in a string.
    // We can use a HashMap where the characters are the keys and the values are the frequencies.
    // For example, given the string "hello", the HashMap would look like this:
    // {'h': 8, 'e': 5, 'l': 12, 'o': 15}

    let result = input
        .lines()
        .map(|line| {
            // we use line over input because we want to iterate over each line instead of each character
            // input is different in that it is a string of all the lines combined
            // line is a string of each line
            let sack_length = line.len() / 2;
            // we divide by 2 because we want to split the line into two equal parts
            // we use .. because we want to take the range from 0 to sack_length; this goes from 0 to sack_length - 1
            let compartment_a = &line[0..sack_length];
            // we use & because we want to borrow the value instead of taking ownership
            // we use .. because we want to take the range from sack_length to the end of the line
            // this goes from sack_length to the end of the line
            let compartment_b = &line[sack_length..(sack_length * 2)];

            let common_char = compartment_a
                .chars()
                // c is a character, it could be any character instead of just being represented by c
                .find(|c| compartment_b.contains(*c))
                .unwrap();
            // this should always return a character becuase we know that there is a common character, which is why we use .unwrap()
            // when we say common character, we mean a character that is in both compartment_a and compartment_b
            letter_scores.get(&common_char).unwrap()
        })
        // because of the way we defined letter_scores, we know that the result will be a number, but we can't assign a u32 because it is smaller than usize
        .sum::<usize>();
    // we use .unwrap because we know that the result will be a number (option type)
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let result = input
        .lines()
        .collect::<Vec<&str>>()
        // rust docs for .chunks: https://doc.rust-lang.org/std/primitive.slice.html#method.chunks
        // here, we are splitting the vector into chunks of 3, for example, if we had a vector of 9 elements, we would split it into 3 chunks of 3
        .chunks(3)
        // we are mapping over each group of 3 lines
        .map(|group| {
            // assign each line to a variable
            let line1 = group[0];
            let line2 = group[1];
            let line3 = group[2];
            // we are finding the common character between line1, line2, and line3
            let common_char = line1
                .chars()
                .find(|c| line2.contains(*c) && line3.contains(*c))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        // because of the way we defined letter_scores, we know that the result will be a number, but we can't assign a u32 because it is smaller than usize
        // we sum over the vector of numbers
        .sum::<usize>();
    // we use .unwrap because we know that the result will be a number (option type)
    result.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
        print!("Part 1 test works")
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
        print!("Part 2 test works")
    }
}
