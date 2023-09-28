use std::collections::BTreeMap;

use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use rayon::prelude::*;

// need Ord to be able to use BTreeMap
// need PartialOrd to be able to use BTreeSet
// need Eq and PartialEq to be able to use BTreeSet
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Sensor {
    x: i32,
    y: i32,
}

// need Debug to be able to use dbg!
#[derive(Debug, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
}

// this function parses the input into the position of the sensors, which is a pair of i32
fn position(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}

// this function parses the input into a BTreeMap of sensors and their closest beacon
// a BTreeMap is a map that is sorted by key, in this case the key is the sensor
// we could just use tuples instead of just BTreeMap but it is easier to read and understand
fn map(input: &str) -> IResult<&str, BTreeMap<Sensor, Beacon>> {
    let (input, list) = separated_list1(
        line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(
                // maps the input into a Sensor struct of the x and y coordinates
                position.map(|(x, y)| Sensor { x, y }),
                tag(": closest beacon is at "),
                // maps the input into a Beacon struct of the x and y coordinates
                position.map(|(x, y)| Beacon { x, y }),
            ),
        ),
    )(input)?;

    Ok((
        input,
        // use into_iter to be able to collect into a BTreeMap over iter() because iter() returns a reference, not owned values
        list.into_iter().collect::<BTreeMap<Sensor, Beacon>>(),
    ))
}

// takes in a &str and line_number and returns a String
// the line_number is the y coordinate of the line we want to check
pub fn process_part1(input: &str, line_number: i32) -> String {
    // parse the input into a BTreeMap of sensors and their closest beacon
    // we assign map to the BTreeMap because we want to use it later
    let (_, map) = map(input).unwrap();
    // assign distances to an i32
    // distances is a BTreeMap of a reference to a Sensor (from the map function) and an i32
    let distances: BTreeMap<&Sensor, i32> = map
        .iter()
        // map the tuple of (sensor, beacon) to (sensor, distance) which is the absolute value of the difference between the x and y coordinates of the sensor and beacon (manhattan distance)
        .map(|(sensor, beacon)| {
            (
                sensor,
                ((beacon.x - sensor.x).abs() + (beacon.y - sensor.y).abs()),
            )
        })
        // collect into a BTreeMap
        .collect();
    // let line_number = 10;

    // x_positions is a Vec<i32> of the x coordinates of the positions on the line that are not blocked by a beacon
    // distances
    let x_positions = distances
        .iter()
        // filters the sensor, distance tuple to only the sensors that are on the line (taking in a closure from the distances BTreeMap)
        .filter(|(sensor, distance)| {
            // we double dereference the distance because it is a reference to a reference
            let sensor_range = (sensor.y - **distance)..=(sensor.y + **distance);
            sensor_range.contains(&line_number)
        })
        // we flat_map the sensor, distance tuple to the x coordinates of the positions on the line that are not blocked by a beacon (taking in a closure from the distances BTreeMap)
        .flat_map(|(sensor, max_distance)| {
            // let width = distance * 2 + 1;
            // sensor.y is the y coordinate of the sensor
            let distance_to_line = sensor.y - line_number;
            // let direction_to_line = distance_to_line.signum();

            // assign max_distance_on_line to the max_distance minus the distance to the line (absolute value)
            let max_distance_on_line = max_distance - distance_to_line.abs();

            // this is the range of x coordinates that are not blocked by a beacon
            // sensor.x is the x coordinate of the sensor
            // here, we set the range to be the x coordinate of the sensor minus the max_distance_on_line to the x coordinate of the sensor plus the max_distance_on_line
            (sensor.x - max_distance_on_line)..=sensor.x + max_distance_on_line
        })
        // unique() returns an iterator yielding only the unique elements from the iterator (in this case, the x coordinates) (from itertools)
        // we need this to use filter() because filter() only works on iterators
        .unique()
        .filter(|x| {
            // we use ! to negate the contains function because we want to filter out the x coordinates that are not in the map
            // we use &Beacon to get a reference to the Beacon because the map is a BTreeMap<&Sensor, Beacon> (code above)
            !map.values().contains(&Beacon {
                // we use *x to dereference the x because it is a reference
                x: *x,
                y: line_number,
            })
        })
        .collect::<Vec<i32>>();
    x_positions.len().to_string()
    // could just write this instead as well
    // count()
    // to_string()
}

// distance calculate for all the sensors at least
// distance calculate for every point
pub fn process_part2(input: &str, limit: i32) -> String {
    // (beacon.x * 4_000_000 + beacon.y).to_string()
    let (_, map) = map(input).unwrap();
    let distances: BTreeMap<&Sensor, i32> = map
        .iter()
        .map(|(sensor, beacon)| {
            (
                sensor,
                (beacon.x - sensor.x).abs() + (beacon.y - sensor.y).abs(),
            )
        })
        .collect();
    // assigning possible_beacon_location to a
    let possible_beacon_location = (0..=limit)
        .cartesian_product(0..=limit)
        // Creates a bridge from this type to a ParallelIterator.
        // This is useful to be able to chain together sequential operations with parallel ones.
        // A bridge is a special kind of iterator that is lazy and can be split into parallel tasks and executed in parallel.
        // A ParallelIterator is a lazy iterator that can process items in parallel.
        .par_bridge()
        .find(|(y, x)| {
            if y < &0 || x < &0 || y > &limit || x > &limit {
                return false;
            }
            // if it's a beacon, then it's not a possible beacon location
            let is_beacon = map.values().contains(&Beacon { x: *x, y: *y });
            if is_beacon {
                return false;
            }
            // doing all of our distances, which are the sensors and their distances that they can reach
            // we are filtering out the sensors that are not in range of the y coordinate
            let is_sensed = distances
                .iter()
                // using a closure of |(sensor, distance)| to filter out the sensors that are not in range of the y coordinate
                .filter(|(sensor, distance)| {
                    // assigning sensor_range to a range of the y coordinate of the sensor minus the distance to the y coordinate of the sensor plus the distance
                    let sensor_range = (sensor.y - **distance)..(sensor.y + **distance);
                    // filtering out anything that isn't in range of y (anything that is not in the range of the sensor)
                    sensor_range.contains(&y)
                })
                .find(|(sensor, max_distance)| {
                    // let width = distance * 2 + 1;
                    let distance_to_line = sensor.y - y;

                    let max_distance_on_line = **max_distance - distance_to_line.abs();

                    let sensor_range =
                        (sensor.x - max_distance_on_line)..(sensor.x + max_distance_on_line);
                    sensor_range.contains(x)
                });
            // if position is not sensed by sensor
            // then we get possible beacon location
            // if it is sensed by sensor, then we don't get possible beacon location (is_none())
            is_sensed.is_none()
        });
    let Some(beacon) = possible_beacon_location else {
        panic!("noooo")
    };
    (beacon.1 * 4000000 + beacon.0).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT, 10), "26");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT, 20), "56000011");
    }
}

// this version has takes too long to run since it has about 16 trillion calculations
