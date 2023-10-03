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
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

// enum Minerals {
//     Ore,
//     Clay,
//     Obsidian,
//     Geode,
// }

#[derive(Debug)]
struct ObsidianRequirements {
    ore: usize,
    clay: usize,
}

#[derive(Debug)]
struct GeodeRequirements {
    ore: usize,
    obsidian: usize,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: ObsidianRequirements,
    geode: GeodeRequirements,
}

// this parses the input for the blueprint part
// this is different from fn blueprints because this parses the input for a single blueprint, while fn blueprints parses the input for multiple blueprints
fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    // this parses the id of the blueprint
    let (input, id) = delimited(tag("Blueprint "), complete::u64, tag(":"))(input)?;
    // this parses the ore cost of the blueprint
    let (input, ore) =
        delimited(tag(" Each ore robot costs "), complete::u64, tag(" ore."))(input)?;
    // this parses the clay cost of the blueprint
    let (input, clay) =
        delimited(tag(" Each clay robot costs "), complete::u64, tag(" ore."))(input)?;
    // this parses the obsidian cost of the blueprint
    let (input, obsidian) = delimited(
        tag(" Each obsidian robot costs "),
        separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
            ObsidianRequirements {
                ore: pair.0 as usize,
                clay: pair.1 as usize,
            }
        }),
        tag(" clay."),
    )(input)?;
    // this parses the geode cost of the blueprint
    let (input, geode) = delimited(
        tag(" Each geode robot costs "),
        separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
            GeodeRequirements {
                ore: pair.0 as usize,
                obsidian: pair.1 as usize,
            }
        }),
        tag(" obsidian."),
    )(input)?;
    // if successful, return the blueprint
    Ok((
        input,
        Blueprint {
            id: id as usize,
            ore: ore as usize,
            clay: clay as usize,
            obsidian,
            geode,
        },
    ))
}

fn blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(line_ending, blueprint)(input)
}

#[derive(Debug, Clone)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

// this implementation has the functions for the Resources struct that allows it to try to build the geode, obsidian, clay, or ore
impl Resources {
    fn run(&mut self) {
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.geode += self.geode_bots;
    }
    // the resources required to build the geode
    fn try_build_geode(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.geode.ore && self.obsidian >= blueprint.geode.obsidian {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.geode.ore;
            new_resources.obsidian -= blueprint.geode.obsidian;
            new_resources.run();
            new_resources.geode_bots += 1;
            Some(new_resources)
        } else {
            None
        }
    }
    // the resources required to build the obsidian
    fn try_build_obsidian(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.obsidian.ore
            && self.clay >= blueprint.obsidian.clay
            && self.obsidian_bots < blueprint.geode.obsidian
        {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.obsidian.ore;
            new_resources.clay -= blueprint.obsidian.clay;
            new_resources.run();
            new_resources.obsidian_bots += 1;
            Some(new_resources)
        } else {
            None
        }
    }
    fn try_build_clay(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.clay && self.clay_bots < blueprint.obsidian.clay {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.clay;
            new_resources.run();
            new_resources.clay_bots += 1;
            Some(new_resources)
        } else {
            // println!(
            //     "couldn't buy obsidian requiring {:?}\n with,\n {:?}",
            //     blueprint.obsidian, self
            // );
            None
        }
    }
    // the resources required to build the ore
    fn try_build_ore(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.ore
            && self.ore_bots
                < blueprint
                    .clay
                    .max(blueprint.obsidian.ore)
                    .max(blueprint.geode.ore)
        {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.ore;
            new_resources.run();
            new_resources.ore_bots += 1;
            Some(new_resources)
        } else {
            None
        }
    }
}

// implementation for the Resources struct that allows the initial resources to be set with 1 (the ore bot) or the default function, which is the default value of 0
impl Default for Resources {
    fn default() -> Self {
        Self {
            ore: Default::default(),
            clay: Default::default(),
            obsidian: Default::default(),
            geode: Default::default(),
            // ore_bots is 1 because we start with 1 ore bot to kcikstart the whole operation
            ore_bots: 1,
            clay_bots: Default::default(),
            obsidian_bots: Default::default(),
            geode_bots: Default::default(),
        }
    }
}

// step_blueprint is a function that takes a blueprint, resources, and time_left, and returns a vector of resources that are the result of the blueprint being stepped (or run) for the given time_left
// takes in the Blueprint, Resources, and usize (time_left) and returns a vector of Resources that are the result of the blueprint being stepped (or run) for the given time_left
fn step_blueprint(blueprint: &Blueprint, resources: Resources, time_left: usize) -> Vec<Resources> {
    // if there is time left, then we can build, collect, and get new bots
    // the time_left.checked_sub(1) is the time_left minus 1, and if it is greater than 0, then we can build, collect, and get new bots
    // Checked integer subtraction. Computes self - rhs, returning None if overflow or underflow occurred.
    if let Some(time_left) = time_left.checked_sub(1) {
        // build, collect plus new bots
        let new_resources = match (
            resources.try_build_geode(blueprint),
            resources.try_build_obsidian(blueprint),
            resources.try_build_clay(blueprint),
        ) {
            // the .. means that the rest of the tuple is ignored because we only care about the first couple of values
            // this returns the resources if the resources are built in the order of geode, obsidian, and clay
            // the values will either be None or Some Vec of Resources
            // this adds the resources to the vector if the resources are built in the order of geode, obsidian, and clay
            // the vector comes from the resources.try_build_geode(blueprint) function
            // it's in the order (geode, obsidian, clay)
            (Some(resources), ..) =>
            // println!("Bought geode");
            {
                Some(resources)
            }
            // this returns the resources if the resources are built in the order of obsidian and clay
            (None, Some(resources), ..) =>
            // println!("Bought obsidian");
            {
                Some(resources)
            }
            // this returns the resources if the resources are built in the order of clay
            (None, None, Some(resources)) =>
            // println!("Bought clay");
            {
                Some(resources)
            }
            _ => None,
        };
        [
            // this tries to build the ore with the resources and then runs the blueprint with the new resources and time_left
            // we use brackets to make it an array so that we can use the .into_iter() function
            resources
                .try_build_ore(blueprint)
                .map(|new_resources| step_blueprint(blueprint, new_resources, time_left)),
            new_resources.map(|new_resources| step_blueprint(blueprint, new_resources, time_left)),
            Some(step_blueprint(
                blueprint,
                {
                    let mut new_resources = resources.clone();
                    new_resources.run();
                    new_resources
                },
                time_left,
            )),
        ]
        .into_iter()
        .filter_map(|v| v)
        .flatten()
        .collect()
    } else {
        // if there is no time left, then we can't build, collect, or get new bots
        vec![resources]
        // done
    }
}

// in process_part1, we parse the input, and then we step the blueprint for 24 hours, and then we get the maximum geode value
pub fn process_part1(input: &str) -> String {
    let (_, blueprints) = blueprints(input).unwrap();
    let maxes: usize = blueprints
    // using par_iter from the rayon crate, we can parallelize the iteration
    // how it works is that it splits the iterator into chunks, and then it runs each chunk in parallel (it's used just like the normal iter function)
    // it's worth parallelizing what we can
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            // dbg!(blueprint);
            let max = step_blueprint(&blueprint, Resources::default(), 24)
                .iter()
                .map(|v| v.geode)
                .max()
                .unwrap();
            // dbg!(result);
            let max = (i + 1) * max;
            println!("{i}:{}", max);
            max
        })
        .sum::<usize>();
    maxes.to_string()
}

// for the second part, we need to find the maximum number of geodes that can be produced in 24 hours
// we don't have to worry about the quality levels. We just worry about the first three blueprints, determine the largest number of geodes you could open; then, multiply these three values together.
pub fn process_part2(input: &str) -> String {
    let (_, blueprints) = blueprints(input).unwrap();
    let maxes: usize = blueprints[0..3]
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let max = step_blueprint(
                &blueprint,
                Resources::default(),
                32, // minutes
            )
            .iter()
            .map(|v| v.geode)
            .max()
            .unwrap();
            // dbg!(result);
            println!("{i}:{}", max);
            max
        })
        .product::<usize>();
    maxes.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "33");
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            process_part2(INPUT),
            (62 * 56).to_string()
        );
    }
}