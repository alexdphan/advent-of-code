use std::{collections::BTreeMap, fmt::Display};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

#[derive(Debug)]
enum Rock {
    Rock,
    Gap,
}

#[derive(Debug)]
struct RockFormation {
    rocks: Vec<Vec<Rock>>,
    // just how far from (0, 0) where the rock is
    offsets: Vec<(usize, usize)>,
}

impl Display for RockFormation {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rocks
                .iter()
                .map(|row| row
                    .iter()
                    .map(|rock| {
                        match rock {
                            Rock::Rock => "#",
                            Rock::Gap => ".",
                        }
                    })
                    .collect::<String>())
                .join("\n")
        )
    }
}

impl RockFormation {
    // can be u64, otherwise we can just stick with usize
    // this gives use the height of the set of rocks
    fn height(&self) -> usize {
        self.rocks.len() as usize
    }
    // this gives use the max_width of the set of rocks
    // we need to move the rocks right and left, but the board also has a size limit of 7 cells wide
    // you can do the code below, or you could do an iteration over the points and get the largest x value
    // here, we just get the longest side
    fn max_width(&self) -> usize {
        self.rocks
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|rock| match rock {
                        Rock::Rock => true,
                        Rock::Gap => false,
                    })
                    .count()
            })
            .max()
            .unwrap()
    }
}

// Returning a Vec of Vec of Rocks
// this is the rocks parser that has a set of moves (enum Move) that will be used to move the rocks
fn rocks(input: &str) -> IResult<&str, Vec<RockFormation>> {
    separated_list1(
        tag("\n\n"),
        separated_list1(
            line_ending,
            many1(alt((
                // turn the char into a Rock
                complete::char('#').map(|_| Rock::Rock),
                // turn the char into a Gap
                complete::char('.').map(|_| Rock::Gap),
            ))),
        )
        .map(|rocks| RockFormation {
            // doing offsets calculation here, iterating over each of the vecs and getting the index and the row
            offsets: rocks
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                  row.iter().enumerate().filter_map(
                        move |(x, r)| match r {
                            Rock::Rock => Some((x, y)),
                            Rock::Gap => None,
                        },
                    )
                })
                // collect into a vec of (x, y) tuples (points or positions)
                .collect::<Vec<(usize, usize)>>(),
            rocks,
        }),
    )(input)
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

// this is the moves parser that has a set of moves (enum Move) that will be used to move the rocks
fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    // many1: this parser will gather and return a Vec of Moves based on the input
    many1(alt((
        complete::char('<').map(|_| Move::Left),
        complete::char('>').map(|_| Move::Right),
    )))(input)
}

// type alias for the field won't work because we want to add new implentations to the BTreeMap
// ex: type Filed = BTreeMap<(usize, usize), Rock>;
// so we create a new struct, a new type pattern
struct Field(BTreeMap<(usize, usize), Rock>);
impl Field {
    // getting the highest y index through all the rocks (points)
    fn highest_rock_y(&self) -> usize {
        // we use *self.0 because it's the only element inside the field struct; returns a reference to a usize, so we dereference it to just return the usize
        // go through all the keys, map over that to get all the y-values, and then get the max value
        // we either have a value or we have 0
        *self.0.keys().map(|(_, y)| y).max().unwrap_or(&0)
    }

    // this is a function that will check if we can place a rock at a certain position
    // takes all of the offsets of the rock formation type and checks if we can place the rock at the desired next position
    fn can_place_rock_at(
        &self,
        rock: &RockFormation,
        desired_next_position: (usize, usize),
    ) -> bool {
        rock.offsets.iter().all(|(x, y)| {
            self.0
             .get(&(
                    desired_next_position.0 + x,
                    desired_next_position.1 - y,
                ))
                .is_none()
        })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_rock_height = self.highest_rock_y();
        let y_range = 0..=max_rock_height;
        let x_range = 0..=7;
        // setting up y, iterating over the x_range
        // iterate over each chunk of 7 and map to match the rock to the string
        let results = y_range
            .rev()
            .cartesian_product(x_range)
            .chunks(7)
            .into_iter()
            .map(|chunk| {
                chunk
                    .map(|(y, x)| {
                        match self.0.get(&(x, y)) {
                            Some(rock) => match rock {
                                // translate into a # or .
                                Rock::Rock => "#",
                                Rock::Gap => ".",
                            },
                            // if there's no rock, then we just return a .
                            None => ".",
                        }
                        // collect into a string
                    })
                    .collect::<String>()
            })
            // join the chunks with a new line
            .join("|\n|");
        // write the results to the formatter
        // we do + on either side of the 7 dashes to make the top and bottom of the field
        write!(f, "|{}|\n+{}+", results, "-".repeat(7))
    }
}

pub fn process(input: &str, rock_limit: usize) -> String {
    let (_, rocks) = rocks(ROCKS).unwrap();
    let (_, moves) = moves(input).unwrap();
    dbg!(moves.len());
    // this allows us to cycle through the rocks and moves infinitely (to repeat the pattern) - referring to .iter().cycle()
    // iterate through a cycle; ex: if you have 5 rocks, the 6th rock will be the same as the first rock
    let mut rocks = rocks.iter().cycle();
    let mut moves = moves.iter().cycle();
    let mut field: Field = Field(BTreeMap::new());
    // requires we have a ground here (we have to have a rock at the bottom), otherwise we get an overflow error
    for x in 0..7 {
        // we insert at each of the x values, and the y value is 0
        field.0.insert((x, 0), Rock::Rock);
    }

    // setting rocks_stopped to 0 because we haven't started yet (initially)
    let mut rocks_stopped: usize = 0;

    // this is the rock limit, which is 2022; that means we have to stop at 2022 rocks falling

    // let rock_limit = 2022;

    // while rocks_stopped is less than 2022, we will continue to iterate through the rocks and moves
    // let us iterate through the rocks infinitely
    while rocks_stopped != rock_limit {
        println!("rocks_stopped: {rocks_stopped}");

        // get the highest rock value from the field
        let max_rock_height = field.highest_rock_y();
        // get the current rock, which is the next rock in the cycle
        let current_rock = rocks.next().unwrap();

        // set the current rock position to be 2, max_rock_height + 3 + the height of the current rock
        let mut current_rock_position: (usize, usize) =
        // current rock position is specifically offset by 2, max_rock_height + 3 + the height of the current rock
        // we need our rock to spawn 3 spaces above the highest rock and because we are using (0, 0) at the top left hand side of our rocks, we need to add the current rock height to the max_rock_height to properly place the rock
            (2, max_rock_height + 3 + current_rock.height());
        // loop through the moves (letting us iterate through the moves infinitely)
        loop {
            // get the next move
            let next_move = moves.next().unwrap();
            // get the current position, and then match on the next move
            let current_position = match next_move {
                Move::Left => {
                    // if we can't move left, then we just return the current rock position
                    // the checked_sub() function will return None if the subtraction would result in an overflow since we are using usize (since we used u32 for the board size, it would use u32 here)
                    if let Some(x_pos) = current_rock_position.0.checked_sub(1) {
                        // if we can't place the rock at the desired next position, then we just return the current rock position
                        let desired_next_position = (
                            x_pos,
                            // .1 is the y value
                            current_rock_position.1,
                        );
                        // if we can't place the rock at the desired next position, then we just return the current rock position
                        if !field.can_place_rock_at(current_rock, desired_next_position) {
                            current_rock_position
                        } else {
                            // otherwise, we return the desired next position
                            desired_next_position
                        }
                    } else {
                        // otherwise, we return the current rock position
                        current_rock_position
                    }
                }
                Move::Right => {
                    // if we can't move right, then we just return the current rock position
                    let desired_next_position = (
                        // .0 is the x value, the + 1 is moving the rock to the right
                        current_rock_position.0 + 1,
                        current_rock_position.1,
                    );
                    // if the current rock position is at the edge of the field, then we just return the current rock position
                    // if the current rock position is 7 - the max width of the current rock, then we just return the current rock position (because we can't move right anymore due to the field size)
                    // or, if we can't place the rock at the desired next position, then we just return the current rock position (if we can, then we return the desired next position)
                    if current_rock_position.0 == 7 - current_rock.max_width()
                        || !field.can_place_rock_at(current_rock, desired_next_position)
                    {
                        current_rock_position
                    } else {
                        desired_next_position
                    }
                }
            };

            // after applying the left and right move, we need to apply the same thing for downward

            // drop downward until we hit a rock
            // drop downward
            let desired_next_position = (current_position.0, current_position.1 - 1);
            
            if field.can_place_rock_at(current_rock, desired_next_position) {
                // set next position
                current_rock_position = desired_next_position;
            } else {
                // field.
                // if we can't place the rock at the desired next position, then we take all of the offsets of the current rock and insert them into the BTreeMap because the rock is going to stop
                for position in current_rock.offsets.iter() {
                    field.0.insert(
                        (
                            position.0 + current_position.0,
                            // moving down, so we subtract the y value
                            current_position.1 - position.1,
                        ),
                        Rock::Rock,
                    );
                }
                // once we inserted all the rock positions onto the field, we increment the rocks_stopped by 1
                rocks_stopped += 1;
                // break out of the loop
                break;
            }
        }
    }
    // return the highest rock y value as a string
    field.highest_rock_y().to_string()
}

// Input the process function into the process_part1 and process_part2 functions

pub fn process_part1(input: &str) -> String {
    process(input, 2022)
}
pub fn process_part2(input: &str) -> String {
    process(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    
    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "3068");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "93");
    }
}
