// we don't need nom because we don't need to parse anything today
use std::collections::BTreeSet;

pub fn process_part1(input: &str) -> String {
    let window_size = 4;

    // collect the input into a vector of chars
    let chars = input.chars().collect::<Vec<char>>();
    // windows() is a function that returns an iterator over all the windows of a given size
    // enumerate() is a function that returns an iterator over all the elements of a collection, but it also returns the index of the element
    let sequence = chars
        .windows(window_size)
        .enumerate()
        // _i is the index of the element, slice is the slice of the vector (which would be a window of size 4)
        // we are using (_i, slice) because enumerate() returns a tuple (index, slice
        .find(|(_i, slice)| {
            // HashSet is a data structure that only stores unique values (Hash Set)
            // BTreeset is a data structure that only stores unique values, but it also keeps them sorted (Binary Tree Set)
            // example: [a, b, c, d, e, f, g, h, i, j, k, l, m, n]
            let set = slice.iter().collect::<BTreeSet<&char>>();
            // if the length of the slice is equal to the length of the set, it means that all the elements of the slice are unique
            slice.len() == set.len()
        })
        // unwrap() is a function that returns the value of an Option, but it panics if the Option is None
        .unwrap();
    // we return the index of the sequence + the window size, because the index is the index of the first element of the sequence
    // we do this because we want the index of the last element of the sequence
    // then we convert the number to a string
    (sequence.0 + window_size).to_string()
}
// do the same thing as process_part1, but we have a window size of 14
pub fn process_part2(input: &str) -> String {
    let window_size = 14;

    let chars = input.chars().collect::<Vec<char>>();
    let sequence = chars
        .windows(window_size)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        })
        .unwrap();
    (sequence.0 + window_size).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(process_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn part2_works() {
        //qmgbljsphdztnv
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
