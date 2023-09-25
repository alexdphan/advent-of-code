use std::{collections::VecDeque, fmt::Display, vec};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, multispace1, newline},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
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

fn grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        newline,
        alpha1.map(|letters: &str| letters.chars().collect()),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    // input the string, and get back a grid (which is a vector of vectors of chars)
    let (_, grid) = grid(input).unwrap();

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| match c {
                    // marks the start of the path (the backtick character from ASCII sort order)
                    'S' => '`',
                    // marks the end of the path (the curly brace character from ASCII sort order)
                    'E' => '{',
                    v => *v,
                })
                .collect()
        })
        .collect();

    // v and c can be anything, but v is a vector of vectors of chars, and c is a char
    // we need double reference because we're iterating over a vector of vectors (hence the comment above)
    let start = grid
        // iter() and iterator is a method that returns an iterator over the vector (which is a vector of vectors of chars)
        .iter()
        // enumerate() is a method that returns an iterator over the vector, where each element is a tuple of the index and the value
        .enumerate()
        // for each element in the vector, get the index and the value and zip them together
        // .zip(std::iter::repeat(i)) is a way to get the index of the outer vector (which is a vector of vectors of chars) ('Zips up' two iterators into a single iterator of pairs)
        // .flat_map is a method that takes a closure and returns an iterator over the results of the closure
        // i is the index of the outer vector (which is a vector of vectors of chars), and v is the value of the outer vector (which is a vector of chars). the v is a vector of chars because we called collect() on the iterator over the chars
        // zip takes the iterators and gives us a tuple of the index and the value (all of the values in the vector of vectors of chars)
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        // Flatten the 2D grid to find the coordinates (x, y) of the element with character 'S'.
        // Unwrap the Option, panicking if 'S' is not found.
        // x value of the character in the first tuples, and the y value as the second value of the contained tuple
        .find_map(|((x, &c), y)| {
            // if it's the backtick character, return the x and y position of the character
            if c == '`' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();
    // do the same thing, but for the end of the path
    let end = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == '{' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    // dbg!(start, end);
    // need itertools for cartesian_product, which allows us to iterate over all the points in the grid
    // assign edges to be the cartesian product of the grid, which is from 0 to the length of the grid as an i32
    // built a vec of the first node and the second node to connect
    // connect the i32, i32, char tuple to the i32, i32, char tuple
    // the id of each node is the tuple, which is the i32, i32, char tuple
    let edges = (0i32..(grid.len() as i32))
        // a cartesian product is the set of all possible ordered pairs from two sets
        .cartesian_product(0i32..(grid[0].len() as i32))
        // flat_map is like map, but flattens the result
        .flat_map(|(y, x)| {
            // assign neighbors to be a vector of tuples of the neighbors of the current cell
            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            // assign current_node_id to be the current cell
            let current_node_id = (x, y);
            // neighbors is a vector of tuples of the neighbors of the current cell
            neighbors
                // iterate over the neighbors
                .iter()
                // filter_map is like map, but filters out the None values (The returned iterator yields only the values for which the supplied closure returns Some(value).)
                .filter_map(|cell| {
                    // for each neighbor, get the cell at that position in the grid
                    // cell.1 is the y position, so get the row at that position in the grid
                    // this is the cell.1 from the tuple of the neighbors of the current cell
                    grid.get(cell.1 as usize)
                        // and then get the cell at that position in the grid (which is cell.0, which is the x position)
                        // The `.and_then()` function is used for optional chaining; it will execute the following closure only if the `Option` is a `Some` variant.
                        // Returns None if the option is None, otherwise calls f with the wrapped value and returns the result.
                        // cell.0 is the x position, so get the cell at that position in the row
                        .and_then(|vec| vec.get(cell.0 as usize))
                        // this part won't get called if the cell is None (from the previous and_then)
                        .and_then(|existing_cell| {
                            // if reachable
                            // get the height of the current cell
                            let current_node_height = grid[y as usize][x as usize];
                            // if the height of the current cell is greater than or equal to the height of the neighbor cell, return the current cell and the neighbor cell
                            // we turn the acii character into a u8, and then compare the u8 values
                            if current_node_height as u8 + 1 >= *existing_cell as u8 {
                                // if true, we return an edge between the current cell and the neighbor cell
                                // the Some of a tuple of two tuples
                                Some((
                                    // this is the first node with the x value, y value, and height of the current cell
                                    (current_node_id.0, current_node_id.1, current_node_height),
                                    // this is the second node with the x value, y value, and height of the neighbor cell (this is the next position it can go to)
                                    (cell.0, cell.1, *existing_cell),
                                ))
                            } else {
                                // if it's not a valid positioin to move to return None
                                None
                            }
                        })
                })
                // collect the results into a vector
                // <Vec<_>> is a type hint, which is needed because the compiler can't infer the type of the vector
                // .flat_map returns an iterator, so we need to collect it into a vector, giving us a single vec of edges (which is a vector of tuples of tuples)
                .collect::<Vec<_>>()
        })
        // collect the results into a vector
        // we collect into (i32, i32, char) because that's what the graph needs
        .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();

    // Create a new undirected GraphMap.
    // Use a type hint to have `()` be the edge weight type.
    // from_edges is a method on UnGraphMap that takes a vector of edges and returns a graph
    // we built a directed graph map with a i32, i32, char tuple as the node id and a () as the edge weight type of unit (which is an empty tuple cause we don't need edge weights)
    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(&edges);
    // dbg!(&graph);

    // printing the graph in dot format, which is a graph description language
    // with_config is a method on Dot that takes a graph and a vector of configs and returns a string
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // let start = (0, 0);
    // let end = (0, 5);

    // dijkstra is a method on Graph that takes a graph, a start node, an end node, and a closure and returns a hashmap
    // [Generic] Dijkstra's shortest path algorithm.
    // Compute the length of the shortest path from start to every reachable node
    // Execute Dijkstra's algorithm on the graph.
    // - The start node is represented by its x and y coordinates, and an additional '`' to denote its height.
    // - The end node is represented by its x and y coordinates, and an additional '{' to denote its height.
    // - The cost for moving from one node to another is constant and equal to 1.
    let res = dijkstra(
        // the graph
        &graph,
        // the start node, which is the start tuple
        (start.0, start.1, '`'), // Start node (x, y, height)
        // the end node, which is the end tuple
        Some((end.0, end.1, '{')), // End node (x, y, height)
        // function that calculates the edge weight, which is a constant cost of 1 (all the edges are worth 1, because it's no harder to go any other node than any other node to get to a particular node)
        |_| 1, // Cost function: constant cost of 1
    );

    // dbg!(&res);
    // after doing dijkstra, we get a hashmap of the shortest path from the start node to the end node
    // the hashmap is a tuple of the end node (which is the end tuple) and the value of the hashmap
    // we can now get the hashmap of node names using the tuple of the end node (which is the end tuple) and value of the hashmap
    res[&(end.0, end.1, '{')].to_string()
    // todo!("part 1");
}

// doing the same thing here, but run this for every a
// we run the jikstra algorithm for every a, and then we get the shortest path for each a
pub fn process_part2(input: &str) -> String {
    let (_, grid) = grid(input).unwrap();

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| match c {
                    'S' => '`',
                    'E' => '{',
                    v => *v,
                })
                .collect()
        })
        .collect();
    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == '`' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();
    // do the same thing, but for the end of the path
    let end = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == '{' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    let edges = (0i32..(grid.len() as i32))
        .cartesian_product(0i32..(grid[0].len() as i32))
        .flat_map(|(y, x)| {
            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            let current_node_id = (x, y);
            neighbors
                .iter()
                .filter_map(|cell| {
                    grid.get(cell.1 as usize)
                        .and_then(|vec| vec.get(cell.0 as usize))
                        .and_then(|existing_cell| {
                            let current_node_height = grid[y as usize][x as usize];
                            if current_node_height as u8 + 1 >= *existing_cell as u8 {
                                Some((
                                    (current_node_id.0, current_node_id.1, current_node_height),
                                    (cell.0, cell.1, *existing_cell),
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();

    // we reverse all the edges
    // we mapped the edges and reversed them
    // we already did all the logic to go from a to b
    // if we reverse them, and start from the end (which is '{' or z), we can find the all the places we can go from them to the end
    // this is because the result of dijkstra is a hashmap of the shortest path from the start node to the end node (below)
    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(edges.iter().map(|(a, b)| (*b, *a)));

    let res = dijkstra(
        &graph,
        // start from the end node, which is the end tuple ('{' or z)
        (end.0, end.1, '{'),
        // make the end None, we can get a list of all the shortest paths from the end node to every other node
        None,
        // Some((end.0, end.1, '{')),
        |_| 1,
    );

    // res[&(end.0, end.1, '{')].to_string()
    let mut results: Vec<i32> = res
        .iter()
        .filter_map(
            // filter_map is like map, but filters out the None values
            // here, we filter the for each node, cost pair, we filter out the nodes have a character that is a
            |(node, cost)| {
                // if the node is 'a', return the cost
                // node.2 means the third value of the tuple, which is the character
                // you could use '`' because it's acii, as long as it's consistent throughout the code
                if node.2 == 'a' {
                    // return the cost
                    Some(*cost)
                } else {
                    // if it's not 'a', return None
                    None
                }
            },
        )
        .collect();
    results.sort();
    results.iter().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "31");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "29");
    }
}
