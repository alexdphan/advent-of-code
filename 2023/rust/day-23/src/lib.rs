use glam::IVec2;
use itertools::Itertools;
use itertools::MinMaxResult::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{alpha1, char, line_ending, one_of, u32},
    combinator::{eof, iterator},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    *,
};
use std::collections::{HashMap, HashSet};
use tracing::*;

fn map(input: &str) -> IResult<&str, HashSet<IVec2>> {
    let mut it = iterator(
        input,
        terminated(many1(one_of(".#")), alt((line_ending, eof))),
    );
    let elves = it
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '.' => None,
                    '#' => Some(IVec2::new(x as i32, y as i32)),
                    _ => panic!("unknown char"),
                })
        })
        .collect::<HashSet<IVec2>>();

    // iterator requires finish to be called
    // res is an IResult<&str, ()>; the <_, _> is a type inference hack (which is to say, I don't know what the type is)
    let res: IResult<_, _> = it.finish();
    Ok((res.unwrap().0, elves))
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let (_, mut field) = map(input).unwrap();
    // checks are the 4 directions to check for a move
    // IVec2 is a 2d vector that is from the glam crate
    // we use this over vec![x,y] because it implements the Add trait (which is required for the iterator below)
    let checks = vec![
        [IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1)],
        [IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)],
        [IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(-1, 1)],
        [IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(1, 1)],
    ];
    let checks_iter = checks.iter().cycle();
    // println!("\nInitial State");
    // print_field(&field);

    for i in 0..10 {
        let local_checks = checks_iter.clone().skip(i).take(4);
        // for check in local_checks.clone() {
        //     println!("check {:?}", check);
        // }

        // proposed_moves is a hashmap of the desired position and the elves that want to move there
        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

        // for each elf, check if they can move to a new position
        // if they can, add them to the proposed_moves hashmap
        // if they can't, add them to the proposed_moves hashmap
        // elf is a reference to the elf's position
        for elf in field.iter() {
            // check for all empty around elf
            // if it is local_checks.clone() then it will check for all empty around elf in all directions
            if local_checks
                .clone()
                // we flatten the array of arrays into a single array of IVec2
                // we iterate over the array of IVec2, then we map each IVec2 to the sum of the IVec2 and the elf, then we flatten the array of IVec2 (flat_map)
                .flat_map(|v| v.iter().map(|vec| *vec + *elf))
                // we remove duplicates
                .unique()
                // Check if all the positions around the elf are empty in the field
                .all(|value| field.get(&value).is_none())
            {
                // If all surrounding positions are empty, add the elf's current position to the proposed_moves HashMap.
                // The key is the elf's current position, and the value is a vector containing the elf's position.
                // Then, skip to the next elf.
                proposed_moves.entry(*elf).or_insert(vec![*elf]);
                continue;
            };
            // Check for a possible move in a direction
            let possible_move = local_checks.clone().find_map(|checks| {
                // If all surrounding positions are empty, output is the elf's current position + the middle position in the checks array
                // If not all positions are empty, output is None
                let output = checks
                    .iter()
                    .all(|position| field.get(&(*position + *elf)).is_none())
                    .then_some(checks[1] + *elf);
                // dbg!(output);
                output
            });
            // If there is a possible move, add the elf's current position to the proposed_moves HashMap.
            // we use r#move because move is a reserved keyword from rust
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            // If there is no possible move, add the elf's current position to the proposed_moves HashMap.
            } else {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }
        // proposed_moves.iter().for_each(|(key, value)| {
        //     println!("{}{:?}", key, value);
        // });

        // The field is updated by iterating over the proposed_moves HashMap.
        // Each entry in the HashMap is a tuple of the desired position and the elves that want to move there.
        field = proposed_moves
            .into_iter()
            .flat_map(|(desired_position, elves_to_move)| {
                // If only one elf wants to move to the desired position, the desired position is added to the new field.
                if elves_to_move.len() == 1 {
                    vec![desired_position]
                } else {
                    // If more than one elf wants to move to the desired position, all the elves are added to the new field.
                    // This is because they cannot move and hence stay in their current positions.
                    elves_to_move
                }
            })
            // The updated positions are collected into a HashSet to form the new field.
            .collect::<HashSet<IVec2>>();
        
        // println!("Round {}", i + 1);
        // print_field(&field);
    }
    let minmax_x = field.iter().map(|v| v.x).minmax();
    let minmax_y = field.iter().map(|v| v.y).minmax();
    let (MinMax(x1, x2), MinMax(y1, y2)) = (minmax_x, minmax_y) else {
        panic!("");
    };

    let min_box_size = (x2 - x1 + 1) * (y2 - y1 + 1);
    (min_box_size as usize - field.len()).to_string()
}

fn print_field(field: &HashSet<IVec2>) {
    let minmax_x = field.iter().map(|v| v.x).minmax();
    let minmax_y = field.iter().map(|v| v.y).minmax();
    let (MinMax(x1, x2), MinMax(y1, y2)) = (minmax_x, minmax_y) else {
        panic!("");
    };
    let output = (y1..=y2)
        .cartesian_product(x1..=x2)
        .map(|(y, x)| match field.get(&IVec2 { x, y }) {
            Some(_) => "#",
            None => ".",
        })
        .chunks((x2 - x1 + 1) as usize)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n");
    println!("{}", output);
}

#[instrument(skip(input))]
pub fn process_part2(input: &str) -> String {
    let (_, mut field) = map(input).unwrap();
    let checks = vec![
        [IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1)],
        [IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)],
        [IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(-1, 1)],
        [IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(1, 1)],
    ];
    let checks_iter = checks.iter().cycle();
    // println!("\nInitial State");
    // print_field(&field);

    let mut rounds = 0;

    for i in 0.. {
        let local_checks = checks_iter.clone().skip(i).take(4);
        // for check in local_checks.clone() {
        //     println!("check {:?}", check);
        // }

        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

        for elf in field.iter() {
            // check for all empty around elf
            if local_checks
                .clone()
                .flat_map(|v| v.iter().map(|vec| *vec + *elf))
                .unique()
                .all(|value| field.get(&value).is_none())
            {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
                continue;
            };
            // check for a possible move in a direction
            let possible_move = local_checks.clone().find_map(|checks| {
                let output = checks
                    .iter()
                    .all(|position| field.get(&(*position + *elf)).is_none())
                    .then_some(checks[1] + *elf);
                // dbg!(output);
                output
            });
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                proposed_moves
                    .entry(*elf)
                    // .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            }
        }
        // proposed_moves.iter().for_each(|(key, value)| {
        //     println!("{}{:?}", key, value);
        // });

        let new_field = proposed_moves
            .into_iter()
            .flat_map(|(desired_position, elves_to_move)| {
                if elves_to_move.len() == 1 {
                    vec![desired_position]
                } else {
                    elves_to_move
                }
            })
            .collect::<HashSet<IVec2>>();
        if field == new_field {
            rounds = i;
            break;
        } else {
            field = new_field
        }
        // println!("Round {}", i + 1);
        // print_field(&field);
    }
    // let minmax_x = field.iter().map(|v| v.x).minmax();
    // let minmax_y = field.iter().map(|v| v.y).minmax();
    // let (MinMax(x1,x2), MinMax(y1,y2)) = (minmax_x,minmax_y) else {
    //     panic!("");
    // };

    // let min_box_size = (x2 - x1 + 1) * (y2 - y1 + 1);
    // (min_box_size as usize - field.len()).to_string()
    (rounds + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    #[ignore]
    fn part1_test_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(
            process_part1(
                ".....
..##.
..#..
.....
..##.
....."
            ),
            "110"
        );
    }
    #[test]
    #[ignore]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "110");
    }

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "20");
    }
}
