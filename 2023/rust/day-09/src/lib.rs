use std::collections::HashSet;
// adds functions to the Iterator trait; anything we can iterate over, we can use the functions in itertools trait
use itertools::Itertools;

use ::lending_iterator::prelude::*;

use ::nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
        map(tag("U"), |_| Direction::Up),
        map(tag("D"), |_| Direction::Down),
    ))(input)?;
    Ok((input, dir))
}

// takes in str input and returns the remaining string and a vector of Directions
fn moves(input: &str) -> IResult<&str, Vec<Direction>> {
    // (input, vecs) is a tuple for the remaining input and the vector of directions
    // parse the input into a vector of tuples of (Direction, u32)
    let (input, vecs) =
        // separated_list1 that takes in a separator (newline) and a parser (separated_pair that takes in a parser (direction()) and a separator (tag(" ")) and a parser (complete::u32)
        separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;
    // output of separated_list1 is a vector of tuples of (Direction, u32)

    // then, we flatten the vector of tuples into a vector of Directions
    let vecs = vecs
        .iter()
        // same thing as doing map(), then flatten()
        .flat_map(|(dir, repeat)| {
            // dir is a reference to a Direction, repeat is a reference to a u32
            // because we're returning a vector of Direction, we need to dereference, do the same to dereference repeat (u32) and cast to usize
            // vector is the thing we want to repeat; the number of times to repeat it
            // if we want to return Direction, we need to do #[derive(Clone, Copy)]
            vec![*dir; *repeat as usize]
        })
        .collect();

    Ok((input, vecs))
}

// Remember, we don't know what the size of the grid is like
pub fn process_part1(input: &str) -> String {
    let (_, move_set) = moves(input).unwrap();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    // Initialize a hash set with the tail position to keep track of visited positions
    // HashSet is a data structure that only stores unique values
    // A hash set implemented as a HashMap where the value is (), in this case, the value is the position of the tail
    let mut tail_positions = HashSet::from([tail]);

    for head_move in move_set.iter() {
        match head_move {
            Direction::Left => {
                head.0 -= 1;
            }
            Direction::Right => {
                head.0 += 1;
            }
            Direction::Up => {
                head.1 += 1;
            }
            Direction::Down => {
                head.1 -= 1;
            }
        }
        let x_range = (head.0 - 1)..=(head.0 + 1);
        let y_range = (head.1 - 1)..=(head.1 + 1);

        // we make a cartesian product of the x and y ranges to get all the points around the head, which is used to check if the tail is in the head's vicinity
        // to get all the ranges, we use the cartesian_product() function from itertools
        // dbg!(x_range
        //     .cartesian_product(y_range)
        //     .collect::<Vec<(i32, i32)>>());

        // we take the tuple for the x and y position for the entire grid, including the position the head is on, and we compare that to the last position of the tail, which will tell us if the tail is still connected to the head
        // if tail is connected, we're done
        let tail_is_connected = x_range
            .cartesian_product(y_range)
            .any(|tuple| tuple == tail);

        // if tail is not connected, we need to move the tail
        // we're not looking as if the the Tail (T) is following the Head (H), we're just seeing the Head and adding the new Tail position to the hash set (which would trail the Head)
        if !tail_is_connected {
            // we already know where the head it, so we can move the tail to the head's position
            // we clone the head and move the tail to the head's position
            let mut new_tail = head.clone();
            match head_move {
                // ex: if the head just moved Left, then the tail needs to move Right
                Direction::Left => {
                    new_tail.0 += 1;
                }
                // ex: if the head just moved Right, then the tail needs to move Left
                Direction::Right => {
                    new_tail.0 -= 1;
                }
                // ex: if the head just moved Up, then the tail needs to move Down
                Direction::Up => {
                    new_tail.1 -= 1;
                }
                // ex: if the head just moved Down, then the tail needs to move Up
                Direction::Down => {
                    new_tail.1 += 1;
                }
            }
            // assign the new tail position to the tail variable
            tail = new_tail;
            // add the new tail position to the hash set
            tail_positions.insert(new_tail);
        }
    }
    // return the number of unique positions in the hash set
    tail_positions.len().to_string()
}

// ---------------------------------- Part 2 ----------------------------------

// for part 2, we need to keep track of the entire rope, not just the head and tail
pub fn process_part2(input: &str) -> String {
    // parse the input into a vector of Directions
    let (_, move_set) = moves(input).unwrap();
    // we need to keep track of the entire rope, so we need to initialize a vector of tuples of (i32, i32) with the length of 10
    let mut rope = [(0, 0); 10];
    // we need to keep track of the tail positions, so we need to initialize a hash set with the tail position
    let mut tail_positions = HashSet::from([*rope.last().unwrap()]);

    // keeping the same logic as part 1, we need to move the head and tail
    for head_move in move_set.iter() {
        match head_move {
            Direction::Left => {
                rope[0].0 -= 1;
            }
            Direction::Right => {
                rope[0].0 += 1;
            }
            Direction::Up => {
                rope[0].1 += 1;
            }
            Direction::Down => {
                rope[0].1 -= 1;
            }
        }

        // we set rope_windows to a mutable iterator of the rope vector with a window size of 2
        // windows_mut() returns an iterator over all contiguous windows of length size. The windows overlap. If the slice is shorter than size, the iterator returns no values.
        // window of 2 means we get a tuple of (head, tail) and we slide the window over by 1 each time
        // the difference between this and windows() is that windows_mut() returns a mutable iterator
        let mut rope_windows = rope.windows_mut::<2>();
        // while we have a head and tail, we need to check if the tail is connected to the head
        while let Some([ref mut head, ref mut tail]) = rope_windows.next() {
            // println!("{:?}{:?}", head, tail);
            let x_range = (head.0 - 1)..=(head.0 + 1);
            let y_range = (head.1 - 1)..=(head.1 + 1);
            // we make a cartesian product of the x and y ranges to get all the points around the head, which is used to check if the tail is in the head's vicinity
            let tail_is_connected = x_range
                .cartesian_product(y_range)
                .any(|tuple| tuple == *tail);
            // if tail is not connected, we need to move the tail
            if !tail_is_connected {
                // println!("{last_head_move:?}");
                // move_tail
                // let mut new_tail = head.clone();
                // if head is on the same x position as tail, then we need to move the tail up or down
                // up if head is above tail, down if head is below tail
                if head.0 == tail.0 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                // else if head is on the same y position as tail, then we need to move the tail left or right
                // left if head is left of tail, right if head is right of tail
                } else if head.1 == tail.1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                // else if head is diagonal to tail, then we need to move the tail to the head's position
                } else {
                    // diagonal
                    // let head_cross_positions = [
                    //     (head.0 - 1, head.1),
                    //     (head.0 + 1, head.1),
                    //     (head.0, head.1 - 1),
                    //     (head.0, head.1 + 1),
                    // ];
                    // the x and y ranges for the head's 3x3 grid
                    // it's a 3x3 grid because the head is in the middle, which we know because the tail is not connected to the head
                    let x_range = (head.0 - 1)..=(head.0 + 1);
                    let y_range = (head.1 - 1)..=(head.1 + 1);
                    // the cartesian product of the x and y ranges for the head's 3x3 grid
                    let head_3x3 = x_range.cartesian_product(y_range).collect::<Vec<_>>();
                    // the new tail position is the intersection of the head's 3x3 grid and the tail's 3x3 grid
                    let x_range = (tail.0 - 1)..=(tail.0 + 1);
                    let y_range = (tail.1 - 1)..=(tail.1 + 1);
                    // maybe_new_tail is a vector of tuples of (i32, i32) that contains the intersection of the head's 3x3 grid and the tail's 3x3 grid
                    let maybe_new_tail: Vec<(i32, i32)> = x_range
                        .cartesian_product(y_range)
                        .filter(|tuple| head_3x3.contains(tuple))
                        .collect();
                    // we match on the length of maybe_new_tail to see if the tail is connected to the head
                    match maybe_new_tail.len() {
                        // if the length is 2, then the tail is connected to the head
                        2 => {
                            // we need to figure out which of the two positions is the next tail position
                            let new_head_cross_positions = [
                                // the new head position is the intersection of the head's 3x3 grid and the tail's 3x3 grid
                                (head.0 - 1, head.1),
                                (head.0 + 1, head.1),
                                (head.0, head.1 - 1),
                                (head.0, head.1 + 1),
                            ];
                            // we need to find the next tail position in the new_head_cross_positions
                            let next = maybe_new_tail
                                .iter()
                                .find(|tuple| new_head_cross_positions.contains(tuple))
                                .unwrap();
                            *tail = *next;
                        }
                        // if the length is 1, then the tail is not connected to the head
                        1 => {
                            *tail = maybe_new_tail[0];
                        }
                        // if the length is anything else, then we have a problem
                        _ => {
                            panic!("unknown tail length");
                        }
                    };
                    // *tail = new_tail;
                }
            }
        }
        // add the new tail position to the hash set
        tail_positions.insert(*rope.last().unwrap());
    }
    tail_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "1");
        assert_eq!(
            process_part2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            "36"
        )
    }
}
