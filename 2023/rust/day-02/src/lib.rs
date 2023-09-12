use std::{cmp::Ordering, str::FromStr};

// The `#` symbol is used for attributes in Rust. Attributes have various uses, including conditional compilation and setting crate name/type.
// The following code defines an enumeration `Move` with three variants: Rock, Paper, and Scissors. Each variant is associated with a unique integer.
// The `PartialOrd` trait is implemented for `Move` to enable comparison between its variants. Using things like `>` and `<` on `Move` variants will now work.
// The `FromStr` trait is implemented for `Move` to enable conversion from string slices to `Move` variants.
// Doing use:: does the same thing, it's purely a style choice.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

// instead of using Ord which uses things like > and < (total ordering), we use PartialOrd which uses things like >= and <= (partial ordering)
impl PartialOrd for Move {
    // The `partial_cmp` method compares two values and returns an `Option<Ordering>`. Takes in a ref to self for the first value (either rock, paper, or scissors), and a ref to the second value to compare (either rock, paper, or scissors)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // This function compares two moves and returns an ordering based on the game rules.
        // If the first move is Scissors and the second move is Rock, Scissors is considered less than Rock.
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        }
        // If the first move is Rock and the second move is Scissors, Rock is considered greater than Scissors.
        else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        }
        // For all other combinations, we compare the numerical values associated with the moves with u8's cmp method.
        else {
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

// & means reference. Read-only, not mutable. Used to avoid copying data (ex: a string) into a function.
pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            // The `split` method splits a string slice on a given pattern, returns an iterator of substrings.
            // Vec is a growable array type. It's a generic type, so it can hold any type, in this case Move.
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            // match is like a switch statement
            // partial_cmp is a method on the Move enum, which returns an Option<Ordering> (Ordering is an enum) that contains Either Less, Greater, or Equal.
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => 0 + moves[1] as u32,
                None => {
                    panic!("Invalid move, moves should be compared")
                }
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();
            let opponent_move = moves[0].parse::<Move>().unwrap();
            // Define a variable `our_move` based on the value of `opponent_move`
            // The `=>` symbol is used in match expressions to separate the pattern from the code to be executed if the pattern matches. => is called the "fat arrow"
            // In Rust, the => syntax is used in pattern matching, specifically within match expressions and similar constructs like if let (if let else).
            // "match" and "=>" are required for pattern matching in Rust.

            // `moves[1]` represents the second element in the `moves` vector, which corresponds to the opponent's move.
            match moves[1] {
                "X" => {
                    let our_move = match opponent_move {
                        // If the opponent's move is Rock (pattern), our move is Scissors (code to be executed)
                        Move::Rock => Move::Scissors,
                        // If the opponent's move is Paper (pattern), our move is Rock (code to be executed)
                        Move::Paper => Move::Rock,
                        // If the opponent's move is Scissors (pattern), our move is Paper (code to be executed)
                        Move::Scissors => Move::Paper,
                    };
                    // Convert our move to its corresponding integer value and add 0 to it
                    0 + our_move as u32
                }
                "Y" => 3 + opponent_move as u32,
                "Z" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    };
                    6 + our_move as u32
                }
                _ => {
                    panic!("Unexpected Response, should be X, Y, or Z")
                }
            }
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    // conventional
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15")
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12")
    }
}
