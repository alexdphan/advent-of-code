use std::{collections::BTreeMap, fs::File, io::Write};

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{eof, iterator},
    multi::separated_list1,
    sequence::{delimited, terminated},
    *,
};
use petgraph::{
    dot::{Config, Dot},
    prelude::DiGraphMap,
    visit::{Topo, Walker},
};
// tracing is a crate that allows us to do logging in a structured way
use tracing::*;

// things we basically have to parse:
// root: pppw + sjmn
// dbpl: 5

// Operation is going to depend on both left and right if it's not a number
#[derive(Debug)]
enum Operation<'a> {
    Number(i64),
    Calculate {
        left: &'a str,
        operator: Math,
        right: &'a str,
    },
}

// str is different than String in Rust. String is a heap allocated string, str is a string slice, which is a reference to a string somewhere else in memory
#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    operation: Operation<'a>,
}

#[derive(Debug)]
enum Math {
    Multiply,
    Add,
    Subtract,
    Divide,
}

#[instrument]
fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, left) = alpha1(input)?;
    let (input, operator) = delimited(
        tag(" "),
        one_of("*+-/").map(|v| match v {
            '*' => Math::Multiply,
            '+' => Math::Add,
            '-' => Math::Subtract,
            '/' => Math::Divide,
            _ => panic!("unknown math operator"),
        }),
        tag(" "),
    )(input)?;
    let (input, right) = alpha1(input)?;
    Ok((
        input,
        Operation::Calculate {
            left,
            operator,
            right,
        },
    ))
}

#[instrument]
fn node(input: &str) -> IResult<&str, Node> {
    // let (input, id) means that the result of the alpha1 parser is stored in the variable id and the rest of the input is stored in the variable input
    // the ? means that we want to return early if the parser fails
    let (input, id) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    // alternates between mapping the i64 parser to Operation::Number (mapping the result of the i64 parser to Operation::Number) and the operation parser (mapping the result of the operation parser to Operation::Calculate
    let (input, operation) =
        alt((complete::i64.map(|num| Operation::Number(num)), operation))(input)?;
    // return the input and the Node struct
    Ok((input, Node { id, operation }))
}

#[instrument(skip(input))]
// DiGraphMap is a directed graph data structure that uses a map to store nodes and a map to store edges
// A GraphMap with directed edges.
// For example, an edge from 1 to 2 is distinct from an edge from 2 to 1.
// DiGraphMap is different from a BTreeMap because it allows us to store multiple values for the same key, while a BTreeMap only allows us to store one value for the same key
fn nodes(input: &str) -> IResult<&str, (BTreeMap<&str, Node>, DiGraphMap<&str, ()>)> {
    // we don't use ? here because we want to return the rest of the input even if the parser fails
    let (input, nodes) = separated_list1(line_ending, node)(input)?;
    // we use flat_map here because we want to flatten the result of the iterator (the result of the iterator is a vector of vectors, we want to flatten it to a vector)
    // every node is going to have two parents or zero parents (the root node is going to have zero parents), so we use flat_map to flatten the result of the iterator
    // the edges are the left and right values of the Calculate operation (the left and right values are the nodes that are connected to the current node)
    // example: root: pppw + sjmn, the left and right values are pppw and sjmn
    // we match the Number operation to an empty vector because the Number operation doesn't have any edges
    // we match the Calculate operation to a vector containing the left and right values
    let edges = nodes.iter().flat_map(|node| match &node.operation {
        // we aren't matching anything named num, so we use _ to ignore it
        // we still need to match the Number operation because we want to return an empty vector if the operation is a Number
        Operation::Number(_num) => vec![],
        Operation::Calculate {
            left,
            // we don't care about the operator, so we use _ to ignore it
            operator: _,
            right,
            // arrays in rust are not the same as arrays in other languages. In rust, arrays have a fixed size (as in, you can't add or remove elements from an array). Vectors are the same as arrays, but they have a dynamic size
            // left and right are double references because we want to return a reference to the id of the node, so we need to dereference the node and then dereference the id
        } => vec![(*left, node.id), (*right, node.id)],
    });
    // assign graph to the result of DiGraphMap::from_edges(edges)
    let graph = DiGraphMap::<&str, ()>::from_edges(edges);

    // let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes()).unwrap();

    // iter() borrows the collection while into_iter() takes ownership of the collection
    // map the nodes to a tuple containing the id and the node
    let nodes = nodes.into_iter().map(|node| (node.id, node)).collect();

    // return the input and the nodes and graph
    Ok((input, (nodes, graph)))
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    // constructed a binary tree and a graph, which is a directed graph (a graph where the edges have a direction)
    // the graph is a directed graph because the edges have a direction (the edges are the left and right values of the Calculate operation)
    // btree is our store of data, graph is our store of relationships between the data
    let (_, (btree, graph)) = nodes(input).unwrap();
    // info!(?graph);
    // Topo comes from petgraph::algo::toposort
    // A topological order traversal for a graph, which means that the nodes are returned in a way that the parent nodes are always returned before the child nodes
    // uses trait called `IntoNeighborsDirected` to get the graph and turns it into the neighbors using the directed information (the directed graph), so we have to pass in the shared reference instead
    // basically, it orders the nodes in a way that the parent nodes are always returned before the child nodes
    // because we have the relationships between the data, we can do a topological traversal of the graph (which means that the parent nodes are always returned before the child nodes)
    let topological = Topo::new(&graph);
    // for every node in the topological order, print the node

    // we need the cache here because it keeps track of the populated values
    let mut cache: BTreeMap<&str, i64> = BTreeMap::new();
    for node_id in topological.iter(&graph) {
        // match &btree.get(node_id) which is a reference to the node id and unwrap it (unwrap the result of the get function)
        // .operation means that we're accessing the operation field of the node
        match &btree.get(node_id).unwrap().operation {
            // Match the Number operation to the num value and insert it into the cache
            Operation::Number(num) => {
                cache.insert(node_id, *num);
            }
            // Match the Calculate operation to the left and right values, then get the values from the cache and perform the operation
            Operation::Calculate {
                left,
                operator,
                right,
            } => {
                // assign left_value to the result of cache.get(left) and unwrap it
                let left_value = cache.get(left).unwrap();
                // assign right_value to the result of cache.get(right) and unwrap it
                let right_value = cache.get(right).unwrap();

                // match the operator to the Math enum and perform the operation
                match operator {
                    Math::Multiply => {
                        cache.insert(node_id, left_value * right_value);
                    }
                    Math::Add => {
                        cache.insert(node_id, left_value + right_value);
                    }
                    Math::Subtract => {
                        cache.insert(node_id, left_value - right_value);
                    }
                    Math::Divide => {
                        cache.insert(node_id, left_value / right_value);
                    }
                }
            }
        }
    }

    cache.get("root").unwrap().to_string()
}

// Part 2 requires a two-stage graph traversal. In the first stage, traverse and perform calculations on the first half of the graph while keeping the second half in memory.
// During the first traversal, store the Node IDs we will need for calculations in the second half of the graph.
// When we reach the root node, reverse any calculations made during the first traversal to obtain the final value needed for the second traversal.
// Now, reconstruct the second half of the graph based on insights gained from the first traversal.
// If certain Node IDs were not encountered during the first traversal, skip them as they pertain to a human-defined path.
// In the second traversal, reverse the direction of edges in the graph and proceed from the root node outward.
// The root node serves as a pivotal point in the algorithm, enabling the switch between the two stages of traversal and the reversal of calculations.

// the objective is to find the number I (humn) yell to pass the root's equality test (root: pppw = sjmn this time)
pub fn process_part2(input: &str) -> String {
    // setting up the same way as part 1
    let (_, (btree, graph)) = nodes(input).unwrap();

    // creating a topological order traversal for the graph
    let topological = Topo::new(&graph);

    // creating a cache for the first traversal
    let mut cache: BTreeMap<&str, i64> = BTreeMap::new();

    // creating the second_graph to have a GraphMap with directed edges
    let mut second_graph = DiGraphMap::<&str, ()>::new();

    // for every node_id in the topological order within the referenced graph, we match teh btree.get(node_id).unwrap().operation to the Operation enum
    for node_id in topological.iter(&graph) {
        match &btree.get(node_id).unwrap().operation {
            Operation::Number(num) => {
                if node_id != "humn" {
                    cache.insert(node_id, *num);
                }
            }
            Operation::Calculate {
                left,
                operator,
                right,
            } => {
                // set the left_value and right_value to the result of cache.get(left) and cache.get(right) and unwrap them
                let left_value = cache.get(left);
                let right_value = cache.get(right);

                // if the node_id is "root", then we match the left_value and right_value
                if node_id == "root" {
                    // if both left_value and right_value are None, then we panic
                    match (left_value, right_value) {
                        // if both left_value and right_value are None, then we panic
                        (None, None) => {
                            panic!("eek2");
                        }
                        // if left_value is None and right_value is Some(r), then we insert the right_value into the cache
                        (None, Some(r)) => {
                            cache.insert(left, *r);
                            continue;
                        }
                        // if left_value is Some(l) and right_value is None, then we insert the left_value into the cache
                        (Some(l), None) => {
                            cache.insert(right, *l);
                            continue;
                        }
                        // if both left_value and right_value are Some, then we panic
                        (Some(_), Some(_)) => panic!("eek"),
                    }
                }
                // after we match the "root", we also match the left_value and right_value
                match (left_value, right_value) {
                    // if both left_value and right_value are Some, then we match the operator to the Math enum and perform the operation
                    (Some(left_value), Some(right_value)) => match operator {
                        Math::Multiply => {
                            cache.insert(node_id, left_value * right_value);
                        }
                        Math::Add => {
                            cache.insert(node_id, left_value + right_value);
                        }
                        Math::Subtract => {
                            cache.insert(node_id, left_value - right_value);
                        }
                        Math::Divide => {
                            cache.insert(node_id, left_value / right_value);
                        }
                    },
                    // if left_value is None and right_value is Some, then we insert the right_value into the cache
                    (Some(_), None) => {
                        // dbg!("a");
                        second_graph.add_edge(node_id, right, ());
                        second_graph.add_edge(left, right, ());
                    }
                    // if left_value is Some and right_value is None, then we insert the left_value into the cache
                    (None, Some(_)) => {
                        // dbg!("b");
                        second_graph.add_edge(node_id, left, ());
                        second_graph.add_edge(right, left, ());
                    }
                    // if both left_value and right_value are None, then we panic
                    (None, None) => {
                        panic!("NoneNone");
                    }
                };
            }
        }
    }

    // let dot = Dot::with_config(
    //     &second_graph,
    //     &[Config::EdgeNoLabel],
    // );
    // // println!(
    // //     "{:?}",
    // //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // // );
    // let mut file = File::create("graph2.dot").unwrap();
    // file.write_all(format!("{:?}", dot).as_bytes())
    //     .unwrap();

    // dbg!(cache.get("root"));
    // dbg!(&second_graph);

    // we do the same thing as the first traversal, but we do it in reverse (we start from the root node and go backwards) in order to get the values for the second traversal
    let topological = Topo::new(&second_graph);
    for node_id in topological.iter(&second_graph) {
        // dbg!(node_id);
        match &btree.get(node_id).unwrap().operation {
            Operation::Number(_num) => {
                // if node_id != "humn" {
                //     dbg!(cache.get(node_id));
                //     // cache.insert(node_id, *num);
                // } else {
                //     // dbg!("calc human", node_id);
                // }
            }
            Operation::Calculate {
                left,
                operator,
                right,
            } => {
                let root_value = cache.get(node_id).unwrap();
                let left_value = cache.get(left);
                let right_value = cache.get(right);

                match operator {
                    Math::Multiply => {
                        match (left_value, right_value) {
                            (None, Some(r)) => {
                                cache.insert(left, root_value / r);
                            }
                            (Some(l), None) => {
                                cache.insert(right, root_value / l);
                            }
                            (None, None) => panic!("eek2"),
                            (Some(_), Some(_)) => {
                                // panic!("eek")
                            }
                        }
                    }
                    Math::Add => match (left_value, right_value) {
                        (None, Some(r)) => {
                            cache.insert(left, root_value - r);
                        }
                        (Some(l), None) => {
                            cache.insert(right, root_value - l);
                        }
                        (None, None) => panic!("eek2"),
                        (Some(_), Some(_)) => {}
                    },
                    Math::Subtract => {
                        // 5 = x - 3; ; x=8; node_id + right_value;
                        // 5 = 3 - x; ; x=-2; * -1; (-1*node_id) - (-1*left_value);
                        match (left_value, right_value) {
                            (None, Some(r)) => {
                                cache.insert(left, root_value + r);
                            }
                            (Some(l), None) => {
                                cache.insert(right, (-1 * root_value) - (-1 * l));
                            }
                            (None, None) => panic!("eek2"),
                            (Some(_), Some(_)) => {
                                // panic!("eek")
                            }
                        }
                    }
                    Math::Divide => {
                        // root = left / right;
                        // 10 = 100 / right
                        // 10 = left / 100
                        match (left_value, right_value) {
                            (None, Some(r)) => {
                                cache.insert(left, root_value * r);
                            }
                            (Some(l), None) => {
                                cache.insert(right, l / root_value);
                            }
                            (None, None) => panic!("eek2"),
                            (Some(_), Some(_)) => {
                                // panic!("eek")
                            }
                        }
                    }
                }
            }
        }
    }

    // dbg!(second_graph);
    // we get the value of humn from the cache, because humn is us that we need to yell for the root to pass the equality test
    cache.get("humn").unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    // we use ignore here becuase you can instantiate with a global subscriber only once (talking here about tracing_subscriber::fmt::init();
    #[test]
    #[ignore]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "152");
    }

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "301");
    }
}

// try RUST_LOG="" cargo run --bin part-1 or RUST_LOG="" cargo run --bin part-2 to run the code without logging

// `rg "use petgraph" ..` to search for a string in all files in the current directory and subdirectories (in this case, we're searching for the string "use petgraph" in all files in the current directory and subdirectories)

// ripgrep is a line-oriented search tool that recursively searches the current directory for a regex pattern while respecting gitignore rules. ripgrep has first class support on Windows, macOS and Linux.
