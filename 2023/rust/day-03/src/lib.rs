use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

pub fn process_part1(input: &str) -> String {
    // chain: https://doc.rust-lang.org/std/iter/struct.Chain.html
    // this is chainning two ranges together
    let letter_scores = ('a'..='z').chain('A'..='Z');
    // here, we are finding the first letter that is 'F' and returning the index
    // |(idx, c)| represents a tuple of (index, character) that we are iterating over
    // in this case, we are finding the first letter that is 'a' and returning the index
    dbg!(letter_scores.enumerate().find_map(|(idx, c)| if c == 'a' {
        {
            Some(idx + 1)
        }
    } else {
        None
    }));
    "result".to_string()
}

pub fn process_part2(input: &str) -> String {
    "result".to_string()
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
        assert_eq!(result, "157")
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12")
    }
}
