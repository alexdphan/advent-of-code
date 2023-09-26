use std::collections::BTreeSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

// this function parses a line of the input into an iterator of (x, y) coordinates
// impl Iterator<Item = (u32, u32)> is a trait bound that says the iterator will return (u32, u32) pairs
fn line(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    // alternating between tag(" -> ") and separated_pair(complete::u32, complete::char(','), complete::u32)
    // (input, pairs) means that the line is a tuple of (input, pairs)
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, complete::char(','), complete::u32),
    )(input)?;
    // would return the line as a set of pairs in (input, pairs), but we want to return an iterator of pairs
    // assign it (iterator) to a variable so we can return it
    let it = pairs
        // to take ownership, we use into_iter() over iter() because iter() would use references to the pairs in the tuple(input, pairs) and not own the values
        // we want the iterator to use the values, so we use into_iter()
        // into_iter is different from iter in that it consumes the values it iterates over
        .into_iter()
        // we use tuple_windows() to get an iterator of pairs of pairs (to destruct the pairs into (ax, ay) and (bx, by))
        // the (ax, ay) and (bx, by) pairs are the coordinates of the rocks; example: ((498, 4), (498, 6))
        .tuple_windows()
        // we use flat_map() to flatten the iterator of pairs of pairs into an iterator of pairs (already set up from tuple_windows())
        // doing a flat_map() gives us a single iterator of pairs instead of an iterator of iterators of pairs
        .flat_map(|((ax, ay), (bx, by))| {
            // getting the min, max, and range for x
            // .min compares two values and returns the smaller one
            // .max compares two values and returns the larger one
            let x_min = ax.min(bx);
            let x_max = ax.max(bx);
            let x_range = x_min..=x_max;

            // getting the min, max, and range for x
            let y_min = ay.min(by);
            let y_max = ay.max(by);
            let y_range = y_min..=y_max;

            // we use cartesian_product() to get an iterator of pairs of (x, y) coordinates
            // getting all the values of x with all the values of y (for each line, so we don't have to worry about the different lines being combined together)
            x_range.cartesian_product(y_range)
        });
    Ok((input, it))
}

// this function parses the input into a BTreeSet of (x, y) coordinates (which are the rocks);
fn rocks(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    // line_ending is newline, but it just accounts for windows systems too (not something we worry about here)
    // Recognizes an end of line (both '\n' and '\r\n').
    // line is a function we made above
    let (input, pairs) = separated_list1(line_ending, line)(input)?;
    // we use a BTreeSet here because it will automatically deduplicate the pairs
    // we use a BTreeSet instead of a HashSet because we want the pairs to be sorted
    // we use a BTreeSet instead of BTreeMap because we don't care about the values (whether they are filled with rock or sand), just the keys
    // have to use into_iter() instead of iter() because we want to take ownership of the pairs; a reference to iter() is not an iter()
    let map = pairs.into_iter().flatten().collect();
    // dbg!(map);
    Ok((input, map))
} // this would return a BTreeSet of (x, y) coordinates (which are the rocks)

pub fn process_part1(input: &str) -> String {
    // getting the board of rocks from the rocks function
    // make board mutable because we are going to change it by inserting the sand
    // we get a BTreeSet into board, which just covers all of the positions that are filled with rock
    let (_, mut board) = rocks(input).unwrap();

    let rock_count = board.len();

    // the board is going to live until the end of the program, so we can use references to it (&(u32, u32))
    // need to make it mutable because we are going to change it
    // iterate the board and collect it into a vector of references to the rocks
    // iterating over board with references to the rocks because we want to sort the rocks by the y coordinate
    let mut rocks_vec = board.iter().collect::<Vec<&(u32, u32)>>();
    // sort the rocks_vec by the y coordinate
    // we compare the y coordinates of the rocks because we want to find the lowest rock
    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));
    // assign lowest_rock to the last rock in the rocks_vec
    // need to use ** because we are getting a reference to a reference, so we dereference twice
    // error[E0502]: cannot borrow `board` as mutable because it is also borrowed as immutable, so we need to use **rocks_vec.last().unwrap() instead of *rocks_vec.last().unwrap() or rocks_vec.last().unwrap()
    // storing the last value into lowest_rock because it was a reference to rocks_vec, which is a reference to board, so we need to store the value in lowest_rock so we can use it later
    // the ** allows lowest_rock to be its own value instead of a reference to a reference, so that we could board.insert(current_sand) later
    let lowest_rock = **rocks_vec.last().unwrap();
    dbg!(lowest_rock);

    // assign the current_sand to (500, 0) because that is where the water starts
    let mut current_sand = (500, 0);
    // use i to keep track of how many iterations we have done
    // let mut i = 1;
    // loop until we reach the bottom of the board
    loop {
        // if the y coordinate of the current_sand is greater than the y coordinate of the lowest_rock, then we have reached the bottom of the board
        if current_sand.1 > lowest_rock.1 {
            // this stops the loop
            break;
        }

        // current_sand.0 is the x coordinate, current_sand.1 is the y coordinate

        // assign down to the coordinate below the current_sand
        let down = (current_sand.0, current_sand.1 + 1);
        // assign left to the coordinate to the left of the current_sand
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        // assign right to the coordinate to the right of the current_sand
        let right = (current_sand.0 + 1, current_sand.1 + 1);
        // match the coordinates below, to the left, and to the right of the current_sand
        match (board.get(&down), board.get(&left), board.get(&right)) {
            // if there is no rock or sand in position where None is, then we can move to down, left, or right
            // the (_, _, _) comes from the match statement above
            (None, _, _) => {
                // valid down move
                current_sand = down;
            }
            (_, None, _) => {
                // valid left move
                current_sand = left;
            }
            (_, _, None) => {
                // valid right move
                current_sand = right;
            }
            // If there is Something in all three positions, then we can't move down, left, or right (we're frozen)
            (Some(_), Some(_), Some(_)) => {
                // i += 1;
                // println!("{}: Frozen at {:?}", i, current_sand);
                // no valid move
                // aka frozen
                board.insert(current_sand);
                current_sand = (500, 0);
            }
        };
    }
    (board.len() - rock_count).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut board) = rocks(input).unwrap();
    let rock_count = board.len();
    let mut rocks_vec = board.iter().collect::<Vec<&(u32, u32)>>();
    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));
    // assigning lowest_rock to the last rock in the rocks_vec
    let lowest_rock = **rocks_vec.last().unwrap();
    dbg!(lowest_rock);
    // if we get the current_sand to lowest_rock, then we can't move down, so we need to stop and return the reference to the lowest_rock (which is the current_sand)
    let mut current_sand = (500, 0);
    // loop {
    // if current_sand.1 > lowest_rock.1 {
    //     break;
    // }
    // using while let instead
    // the loop keeps running while the condition is true (which is while board.get(&(500, 0)) is None)
    while let None = board.get(&(500, 0)) {
        // assigning the coordinates below, to the left, and to the right of the current_sand
        // this is for the match statement below
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right = (current_sand.0 + 1, current_sand.1 + 1);

        // match and doing board.get on each of the potential positions
        match (
            // if the potential positions are empty (marked by None), then we can move down, left, or right
            board.get(&down).or_else(|| {
                // if down hits the floor, then we can't move down so we return the lowest_rock
                // check to see if the potential positions is going to be on the imaginary floor we don't have in the board
                if down.1 == lowest_rock.1 + 2 {
                    // we return Some(&lowest_rock) because we want to return a reference to the lowest_rock
                    // this would just be a reference to the current_sand since we are on the lowest_rock
                    // returns the imaginary floor
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            board.get(&left).or_else(|| {
                // if left hits the floor, then we can't move down so we return the lowest_rock
                if left.1 == lowest_rock.1 + 2 {
                    // we return Some(&lowest_rock) because we want to return a reference to the lowest_rock
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            board.get(&right).or_else(|| {
                // if right hits the floor, then we can't move down so we return the lowest_rock
                if right.1 == lowest_rock.1 + 2 {
                    // we return Some(&lowest_rock) because we want to return a reference to the lowest_rock
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
        ) {
            (Some(_), Some(_), Some(_)) => {
                board.insert(current_sand);
                current_sand = (500, 0);
            }
            (None, _, _) => {
                current_sand = down;
            }
            (_, None, _) => {
                current_sand = left;
            }
            (_, _, None) => {
                current_sand = right;
            }
        };
    }
    (board.len() - rock_count).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "24");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "93");
    }
}
