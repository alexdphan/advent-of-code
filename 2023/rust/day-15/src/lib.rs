use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    *,
};
use std::{
    collections::BTreeMap,
    ops::{Range, RangeInclusive},
};

// we use i64 because it can handle much larger numbers than i32 before overflowing
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Sensor {
    x: i64,
    y: i64,
}

impl Sensor {
    fn distance_to_beacon(&self, beacon: &Beacon) -> i64 {
        (beacon.x - self.x).abs() + (beacon.y - self.y).abs()
    }
    fn y_range(&self, max_distance: i64) -> Range<i64> {
        (self.y - max_distance)..(self.y + max_distance)
    }
    /// check whether a given y-index is reachable by
    /// the given sensor
    fn covers(&self, distance: i64, y_index: i64) -> bool {
        let sensor_range = self.y_range(distance);
        sensor_range.contains(&y_index)
    }
    /// returns a range of the x values, centered on the
    /// sensor's x position, that the sensor can sense
    /// at the target y index
    fn x_coverage_at_y(&self, max_distance: i64, target_y_index: i64) -> RangeInclusive<i64> {
        let distance_to_line = self.y - target_y_index;

        let max_distance_on_line = max_distance - distance_to_line.abs();

        (self.x - max_distance_on_line)..=self.x + max_distance_on_line
    }
    fn coverage_in_x_ranges(&self, max_distance: i64) -> Vec<(i64, RangeInclusive<i64>)> {
        self.y_range(max_distance)
            .map(|y| {
                let x_range = self.x_coverage_at_y(max_distance, y);
                (y, x_range)
            })
            .collect()
    }
}

// need Debug to be able to use dbg!
#[derive(Debug, PartialEq)]
struct Beacon {
    x: i64,
    y: i64,
}

// this function parses the input into the position of the sensors, which is a pair of i32
fn position(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(input)
}
fn map(input: &str) -> IResult<&str, BTreeMap<Sensor, Beacon>> {
    let (input, list) = separated_list1(
        line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(
                position.map(|(x, y)| Sensor { x, y }),
                tag(": closest beacon is at "),
                position.map(|(x, y)| Beacon { x, y }),
            ),
        ),
    )(input)?;

    Ok((
        input,
        list.into_iter().collect::<BTreeMap<Sensor, Beacon>>(),
    ))
}

// merge_ranges takes in an accumulator and a range and returns an accumulator
// this function is used to merge the ranges of the sensors 
fn merge_ranges(
    mut acc: (RangeInclusive<i64>, Option<i64>),
    range: &RangeInclusive<i64>,
) -> (RangeInclusive<i64>, Option<i64>) {
    if acc.1.is_some() {
        return acc;
    }
    if acc.0.end() + 1 >= *range.start() {
        acc.0 = *acc.0.start()..=(*acc.0.end().max(range.end()));
    } else {
        acc.1 = Some(acc.0.end() + 1);
    }

    acc
}

// takes in a &str and line_number and returns a String
// the line_number is the y coordinate of the line we want to check
pub fn process_part1(input: &str, line_number: i64) -> String {
    let (_, map) = map(input).unwrap();
    map.iter()
        .filter_map(|(sensor, closest_beacon)| {
            let distance = sensor.distance_to_beacon(closest_beacon);
            if sensor.covers(distance, line_number) {
                Some(sensor.x_coverage_at_y(distance, line_number))
            } else {
                None
            }
        })
        .flatten()
        .unique()
        .filter(|x| {
            !map.values().contains(&Beacon {
                x: *x,
                y: line_number,
            })
        })
        .count()
        .to_string()
}

// distance calculate for all the sensors at least
// distance calculate for every point
pub fn process_part2(input: &str, limit: i64) -> String {
    let (_, map) = map(input).unwrap();
    // assign ranges to y index to a BTreeMap of i64 to Vec<RangeInclusive<i64>>
    let ranges_by_y_index: BTreeMap<i64, Vec<RangeInclusive<i64>>> = map
        .iter()
        .flat_map(|(sensor, closest_beacon)| {
            let max_distance = sensor.distance_to_beacon(closest_beacon);
            let ranges = sensor.coverage_in_x_ranges(max_distance);
            ranges
                .into_iter()
                .map(|(y, range)| (y, *range.start().max(&0)..=*range.end().min(&limit)))
        })
        .filter(|(y, _)| y >= &0 && y <= &limit)
        .fold(BTreeMap::new(), |mut acc, (y, range)| {
            acc.entry(y)
                .and_modify(|ranges| ranges.push(range.clone()))
                .or_insert(vec![range]);
            acc
        });

    // have (x, y) be assigned to t
    let (x, y) = ranges_by_y_index
        .into_iter()
        .find_map(|(y_index, mut ranges)| {
            ranges.sort_by(|a, b| a.start().cmp(b.start()));
            let result: (RangeInclusive<i64>, Option<i64>) =
                ranges.iter().fold((0..=0, None), merge_ranges);
            result.1.map(|x| (x, y_index))
        })
        .unwrap();

    (x * 4000000 + y).to_string()
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

