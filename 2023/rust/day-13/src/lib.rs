use std::{cmp::Ordering, collections::VecDeque, fmt::Display, vec};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, multispace1, newline},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    *,
};

use petgraph::graphmap::UnGraphMap;
use petgraph::prelude::*;
use petgraph::Graph;
use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
};
use std::collections::HashMap;

use std::cmp::Ordering::*;

// a struct Pair for the two packets in a pair
// PartialEq is for testing, which is a trait that allows us to compare two values for equality
#[derive(Debug, PartialEq)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

// an enum Packet for the different types of packets
// PartialOrd allows us to run cmp() on the enum
// PartialOrd is different from Ord in that it allows us to compare two values, but not necessarily order them
// the reason we can do left cmp() right is because we have derived PartialOrd and Eq (Eq is a marker trait that indicates that two values are equal, just a signifier for the compiler)
#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>), // List looks like [[1,2,3],4,5]
    Number(u32),
}

// this is a trait that allows us to display a value (we implement Display for Packet)
impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // if it's a list, format it as a list
                Packet::List(list) => format!(
                    "[{}]",
                    list.iter()
                        // map(): here, we map the vector of packets to a string
                        .map(|v| v.to_string())
                        // intersperse: insert a separator between each element of the iterator
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Packet::Number(num) => num.to_string(),
            }
        )
    }
}

// this is a trait that allows us to compare two values for equality (we implement PartialEquality for Packet)
// source: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // if both are lists, compare the lists
            // l0 just mean left hand side 0, r0 means right hand side 0 (they are just variables)
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            // if both are numbers, compare the numbers
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            // if one is a list and the other is a number, compare the list to a list with the number
            // we take the first list and wrap the number into a second list to compare the two lists
            // because these lists are references (a reference to a Vec<Packet>), we chose to reference the Vec<Packet> with &vec![Packet::Number(*r0)]
            (Self::List(l0), Self::Number(r0)) => l0 == &vec![Packet::Number(*r0)],
            // if one is a number and the other is a list, compare the list with the number to the list
            (Self::Number(l0), Self::List(r0)) => &vec![Packet::Number(*l0)] == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// if you implement Ord, make sure PartialOrd agrees with Ord

// this is a trait that allows us to compare two values for ordering (we implement Ord for Packet)
// it's very similar to PartialEq, but it returns an Ordering instead of a bool
impl Ord for Packet {
    // cmp() returns an Ordering, which is either Less, Equal, or Greater
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // if both are lists, compare the lists
            // a means left hand side, b means right hand side (they are just variables)
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            // if both are numbers, compare the numbers
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            // if both are numbers, compare the numbers
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(&b),
            // if both are numbers, compare the numbers
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}
// would return Less if the left is less than the right, Equal if the left is equal to the right, and Greater if the left is greater than the right

// difference between impl PartialEq and impl Ord is that PartialEq returns a bool, while Ord returns an Ordering

// a function to parse a packet
pub fn packet(input: &str) -> IResult<&str, Packet> {
    // alt: try the first parser, if it fails, try the second, etc.
    // can either be another list or a number
    alt((
        // the first parser
        // delimited: parse the first parser, then the second, then the third, and return the second
        // map(): here, we map the vector of packets to a Packet::List
        // separated_list0: parse the first parser, then the second, then the first, then the second, etc. until error
        // if there is only one item, we keep looking for another packet inside the two brackets
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        // the second parser
        // u32 to map the number to a Packet::Number
        nom::character::complete::u32.map(|num| Packet::Number(num)),
    ))(input)
}

// a function to parse a pair
pub fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    // separated_list1: alternate between the first and second parser until error
    separated_list1(
        // tag the double newline
        tag("\n\n"),
        // separated_pair: Gets an object from the first parser, then matches an object from the sep_parser and discards it, then gets another object from the second parser.
        // map it to a Pair where the left is the first object and the right is the second object (p1 and p2)
        separated_pair(packet, newline, packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, pair_list) = pairs(input).unwrap();
    pair_list
        .iter()
        .enumerate()
        .filter_map(|(i, Pair { left, right })| match left.cmp(right) {
            // if we cmp in the enum Packet, we can compare the two packets with Ordering Pair that is either Less, Equal, or Greater
            std::cmp::Ordering::Less => Some(i),
            std::cmp::Ordering::Equal => panic!("equal??"),
            std::cmp::Ordering::Greater => None,
        })
        // enumerate() starts at 0, but our answer needs to start at 1, hence the (|v| v + 1)
        .map(|v| v + 1)
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, pair_list) = pairs(input).unwrap();
    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let mut packets: Vec<&Packet> = pair_list
        .iter()
        .flat_map(|Pair { left, right }| [left, right])
        .chain([&packet_2, &packet_6])
        .collect();
    // using sort_by() instead of sort() because sort() requires Ord, but we only have PartialOrd
    // our Ord and PartialOrd (we derived PartialOrd) don't match, so we can't use sort() if we uncomment the line below
    // packets.sort_by(|a, b| a.cmp(b));
    packets.sort();
    println!(
        "{}",
        &packets
            .iter()
            .map(|v| v.to_string())
            // intersperse: insert a separator between each element of the iterator
            .intersperse("\n".to_string())
            .collect::<String>()
    );
    let index_2 = packets
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&packet_2)
        .unwrap();
    let index_6 = packets
        .iter()
        .enumerate()
        .find(|(_i, packet)| packet == &&&packet_6)
        .unwrap();
    dbg!(index_2, index_6);

    // .0 means it returns the index, .1 means it returns the value
    // need to add 1 because enumerate() starts at 0, but our answer needs to start at 1
    ((index_2.0 + 1) * (index_6.0 + 1)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn parser_works() {
        use Packet::*;
        assert_eq!(
            pairs(INPUT).unwrap().1,
            vec![
                Pair {
                    left: List(vec![Number(1), Number(1), Number(3), Number(1), Number(1),]),
                    right: List(vec![Number(1), Number(1), Number(5), Number(1), Number(1),]),
                },
                Pair {
                    left: List(vec![
                        List(vec![Number(1),]),
                        List(vec![Number(2), Number(3), Number(4),]),
                    ]),
                    right: List(vec![List(vec![Number(1),]), Number(4),]),
                },
                Pair {
                    left: List(vec![Number(9),]),
                    right: List(vec![List(vec![Number(8), Number(7), Number(6),]),]),
                },
                Pair {
                    left: List(vec![
                        List(vec![Number(4), Number(4),]),
                        Number(4),
                        Number(4),
                    ]),
                    right: List(vec![
                        List(vec![Number(4), Number(4),]),
                        Number(4),
                        Number(4),
                        Number(4),
                    ]),
                },
                Pair {
                    left: List(vec![Number(7), Number(7), Number(7), Number(7),]),
                    right: List(vec![Number(7), Number(7), Number(7),]),
                },
                Pair {
                    left: List(vec![]),
                    right: List(vec![Number(3),]),
                },
                Pair {
                    left: List(vec![List(vec![List(vec![],),]),]),
                    right: List(vec![List(vec![]),]),
                },
                Pair {
                    left: List(vec![
                        Number(1),
                        List(vec![
                            Number(2),
                            List(vec![
                                Number(3),
                                List(vec![
                                    Number(4),
                                    List(vec![Number(5), Number(6), Number(7),]),
                                ]),
                            ]),
                        ]),
                        Number(8),
                        Number(9),
                    ]),
                    right: List(vec![
                        Number(1),
                        List(vec![
                            Number(2),
                            List(vec![
                                Number(3),
                                List(vec![
                                    Number(4),
                                    List(vec![Number(5), Number(6), Number(0),]),
                                ]),
                            ]),
                        ]),
                        Number(8),
                        Number(9),
                    ]),
                },
            ]
        )
    }

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "140");
    }
}
