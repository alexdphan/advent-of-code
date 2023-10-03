use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
};

use glam::IVec3;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

// this function is used to parse the input of points from the input file
fn points(input: &str) -> IResult<&str, Vec<IVec3>> {
    // alternates until error
    separated_list1(
        // parses the line ending, which is either \n or \r\n
        line_ending,
        // alternates between "," tag and a 3d vector of i32s
        separated_list1(tag(","), complete::i32).map(|vec| IVec3::new(vec[0], vec[1], vec[2])),
    )(input)
}

// this function returns the surface area of the points
pub fn process_part1(input: &str) -> String {
    // parse the input into a vector of 3d vectors
    let (_, points) = points(input).unwrap();
    // convert the vector into a hashset
    let points: HashSet<IVec3> = HashSet::from_iter(points.into_iter());

    // calculate the surface area
    // for each point that we've parses from the points function, calculate the number of free sides
    // free sides are sides that are not in the points HashSet; which means that they are exposed to the surface area
    // maps the number of free sides to the surface area
    // the reference to the 3d vector is dereferenced to get the 3d vector
    // IVec3 is a struct that contains 3 i32s
    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            // Create a new 3D vector with x decreased by 1
            let x_low = IVec3::new(x - 1, y, z);
            // Create a new 3D vector with x increased by 1
            let x_high = IVec3::new(x + 1, y, z);
            // Create a new 3D vector with y decreased by 1
            let y_low = IVec3::new(x, y - 1, z);
            // Create a new 3D vector with y increased by 1
            let y_high = IVec3::new(x, y + 1, z);
            // Create a new 3D vector with z decreased by 1
            let z_low = IVec3::new(x, y, z - 1);
            // Create a new 3D vector with z increased by 1
            let z_high = IVec3::new(x, y, z + 1);

            // Create an array of the above vectors
            // Iterate over the array
            // Filter out vectors that are not in the points HashSet; which is the number of free sides
            // Count the number of vectors that pass the filter, which is the number of sides that are free (None, or not in the HashSet)
            // the number of sides that are free are the number of sides that are exposed to the surface area
            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .count()
        })
        // surface area is the sum of the number of free sides
        .sum::<usize>();

    // convert the surface area to a string
    surface_area.to_string()
}

// part 2 functions

// this function processes the input into a vector of 3d vectors
fn process_block(
    &IVec3 { x, y, z }: &IVec3,
    points: &HashSet<IVec3>,
) -> usize {
    // number of free sides
    // this part is the same as the process_part1 function
    let x_low = IVec3::new(x - 1, y, z);
    let x_high = IVec3::new(x + 1, y, z);
    let y_low = IVec3::new(x, y - 1, z);
    let y_high = IVec3::new(x, y + 1, z);
    let z_low = IVec3::new(x, y, z - 1);
    let z_high = IVec3::new(x, y, z + 1);
    // we iterate over the array of vectors, and filter out vectors that are not in the points HashSet
    // then, we check if the filtered vector is an interior block through the is_interior_block function
    [x_low, x_high, y_low, y_high, z_low, z_high]
        .iter()
        .filter(|ivec| points.get(ivec).is_none())
        .map(|ivec| {
            if is_interior_block(&ivec, &points) {
                // (interior wall, exterior wall)
                (1, 0)
            } else {
                (0, 1)
            }
        })
        .map(|(_interior, exterior)| exterior)
        .sum::<usize>()
}

// the process_part2 function is the same as the process_part1 function, except that it uses the process_block and is_interior_block function to calculate the surface area of the points
// this function returns the surface area of the points that are exterior surface area of the lava droplet
pub fn process_part2(input: &str) -> String {
    let (_, points) = points(input).unwrap();
    let points: HashSet<IVec3> =
        HashSet::from_iter(points.into_iter());

    // this is similar to the process_part1 function, except that we use the process_block function to calculate the surface area of the points and the is_interior_block function to check if a point is an interior block
    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);
            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .map(|ivec| {
                    if is_interior_block(&ivec, &points) {
                        let IVec3 { x, y, z } = *ivec;
                        let x_low = IVec3::new(x - 1, y, z);
                        let x_high =
                            IVec3::new(x + 1, y, z);
                        let y_low = IVec3::new(x, y - 1, z);
                        let y_high =
                            IVec3::new(x, y + 1, z);
                        let z_low = IVec3::new(x, y, z - 1);
                        let z_high =
                            IVec3::new(x, y, z + 1);
                        // (interior wall, exterior wall)
                        let is_really_exterior_block = [
                            x_low, x_high, y_low, y_high,
                            z_low, z_high,
                        ]
                        .iter()
                        .filter(|ivec| {
                            points.get(ivec).is_none()
                        })
                        .any(|block| {
                            process_block(block, &points)
                                >= 1
                        });
                        if is_really_exterior_block {
                            (0, 1)
                        } else {
                            (1, 0)
                        }
                    } else {
                        (0, 1)
                    }
                })
                .map(|(_interior, exterior)| exterior)
                .sum::<usize>()
        })
        .sum::<usize>();

    surface_area.to_string()
}

// this function checks if a 3d vector is an interior block by checking if all of its sides are bounded by other blocks
fn is_interior_block(
    &IVec3 { x, y, z }: &IVec3,
    points: &HashSet<IVec3>,
) -> bool {
    // check if all of the sides of the 3d vector are bounded by other blocks
    let bounded_x_pos = points
    // this iterates over the points in the &IVec3 { x, y, z } 3d vector, and checks if the x value of the point is greater than the x value of the 3d vector, and the y and z values are equal to the y and z values of the 3d vector
    // then does the similar thing for the other sides
        .iter()
        .find(|point| {
            point.x > x && point.y == y && point.z == z
        })
        .is_some();
    let bounded_x_neg = points
        .iter()
        .find(|point| {
            point.x < x && point.y == y && point.z == z
        })
        .is_some();
    let bounded_y_pos = points
        .iter()
        .find(|point| {
            point.x == x && point.y > y && point.z == z
        })
        .is_some();
    let bounded_y_neg = points
        .iter()
        .find(|point| {
            point.x == x && point.y < y && point.z == z
        })
        .is_some();
    let bounded_z_pos = points
        .iter()
        .find(|point| {
            point.x == x && point.y == y && point.z > z
        })
        .is_some();
    let bounded_z_neg = points
        .iter()
        .find(|point| {
            point.x == x && point.y == y && point.z < z
        })
        .is_some();
    [
        bounded_x_pos,
        bounded_x_neg,
        bounded_y_pos,
        bounded_y_neg,
        bounded_z_pos,
        bounded_z_neg,
    ]
    .iter()
    .all(|v| *v)
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "64");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "58");
    }
}
