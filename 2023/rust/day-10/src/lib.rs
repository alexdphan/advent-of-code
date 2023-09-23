use std::{collections::BTreeMap, fmt::Debug};
// adds functions to the Iterator trait; anything we can iterate over, we can use the functions in itertools trait

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

use std::{fmt::Display, ops::RangeInclusive};

struct Computer {
    x: i32,
    cycles: u32,
    pixels: String,
}

impl Display for Computer {
    // this function takes in a reference to self and a reference to a Formatter(from std::fmt) and returns a Result
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!() takes in a reference to a Formatter and a string and returns a Result
        // f: &mut std::fmt::Formatter<'_> means that f is a mutable reference to a Formatter
        // self.pixels.chars() returns an iterator over the characters in self.pixels
        write!(
            f,
            "{}",
            self.pixels
                // .chars() returns an iterator over the characters in self.pixels
                .chars()
                // .chunks(40) returns an iterator over chunks of 40 characters (meaning that each chunk is a line, since each line is 40 characters long)
                .chunks(40)
                // .into_iter() converts the iterator into an iterator that owns its contents
                .into_iter()
                // .map() takes in a closure and returns an iterator that applies the closure to each element
                .map(|chunk| chunk.collect::<String>())
                .join("\n")
        )
    }
}

impl Computer {
    fn new() -> Self {
        Computer {
            x: 1,
            cycles: 0,
            pixels: "".to_string(),
        }
    }
    fn sprite_range(&self) -> RangeInclusive<i32> {
        (self.x - 1)..=(self.x + 1)
    }
    // this function takes in a reference to an instruction and returns nothing
    // &mut self means that the function takes in a mutable reference to self
    fn interpret(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.cycles() {
            // start_cycle() returns a Cycle struct
            let cycle_guard = self.start_cycle();
            // if the sprite_range contains the pixel, push a #, otherwise push a .
            if cycle_guard
                .computer
                .sprite_range()
                .contains(&(cycle_guard.pixel as i32))
            {
                cycle_guard.computer.pixels.push_str("#");
            } else {
                cycle_guard.computer.pixels.push_str(".");
            }
        }
        // match the instruction and do the appropriate action
        match instruction {
            Noop => {}
            Add(num) => {
                self.x += num;
            }
        };
    }

    // this function takes in a reference to self and returns a Cycle struct
    fn start_cycle(&mut self) -> Cycle {
        Cycle {
            cycle: self.cycles,
            pixel: self.cycles % 40,
            computer: self,
        }
    }
}

struct Cycle<'a> {
    cycle: u32,
    pixel: u32,
    computer: &'a mut Computer,
}
impl<'a> Drop for Cycle<'a> {
    fn drop(&mut self) {
        self.computer.cycles += 1;
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

// this allows us to use Noop and Add() without having to type Instruction::Noop and Instruction::Add()
use Instruction::*;

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Noop => 1,
            Add(_) => 2,
        }
    }
}

// a function that takes in an input and returns a Result that contains the reference to the input and a Vec of Instructions
fn instruction_set(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, vecs) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Noop),
            preceded(tag("addx "), complete::i32).map(|num| Add(num)),
        )),
    )(input)?;

    Ok((input, vecs))
}

pub fn process_part1(input: &str) -> String {
    let notable_cycles = [20, 60, 100, 140, 180, 220];
    let mut scores: BTreeMap<u32, i32> = BTreeMap::new();

    let (_, instructions) = instruction_set(input).unwrap();
    // using i32 because we know the result will be small enough to fit in an i32
    let mut x: i32 = 1;
    // using u32 because we know the result will be small enough to fit in a u32
    let mut cycles: u32 = 0;
    for instruction in instructions.iter() {
        if notable_cycles.contains(&(cycles + 1)) {
            scores.insert(cycles + 1, (cycles as i32 + 1) * x);
        }
        if notable_cycles.contains(&(cycles + 2)) {
            scores.insert(cycles + 2, (cycles as i32 + 2) * x);
        }
        cycles += instruction.cycles();
        match instruction {
            Noop => {}
            Add(num) => {
                x += num;
            }
        };
    }

    scores
        .iter()
        .map(|(_key, value)| value)
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    // parse the input into a Vec of Instructions
    let (_, instructions) = instruction_set(input).unwrap();
    // assigning computer to the result of the fold function (which is a Computer that takes in a computer and an instruction and returns a computer)
    let computer = instructions
        .iter()
        // we use fold() to iterate over the instructions and accumulate the result into a Computer.
        // Computer::new() serves as the initial value for the accumulator (i.e., the computer)
        .fold(Computer::new(), |mut computer, instruction| {
            // interpret() updates the state of the computer based on the current instruction
            computer.interpret(instruction);
            // The updated computer is returned for the next iteration
            computer
        });
    // computer.to_string() would produce a string representation of the computer's final state
    computer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13140");
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            process_part2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
