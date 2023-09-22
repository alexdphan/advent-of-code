use nom::character::complete::{digit1, newline};
use nom::multi::separated_list1;
use nom::{IResult, Parser};

// parse each tree in the input
fn parse_trees(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, vecs) = separated_list1(
        newline,
        digit1.map(|nums: &str| nums.chars().map(|num| num.to_digit(10).unwrap()).collect()),
    )(input)?;

    Ok(("input", vecs))
}

pub fn process_part1(input: &str) -> String {
    let (_, trees) = parse_trees(input).unwrap();
    let max_length = trees.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(i, tree_line)| {
            let line_max_length = tree_line.len() - 1;
            tree_line
                .iter()
                .enumerate()
                // Check if the current tree is on the edge of the map:
                // - If it is on the first row or last row (i == 0 or i == max_length)
                // - If it is on the first column or last column (line_i == 0 or line_i == line_max_length)
                // Set the corresponding element in the visible_trees matrix to true, otherwise set it to false.
                .map(|(line_i, _)| {
                    if i == 0 || i == max_length || line_i == 0 || line_i == line_max_length {
                        true
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect();

    // Iterations for Xs (Left to Right and Right to Left)
    for y in 0..trees.len() {
        let mut current_tree_size = 0;
        for x in 0..trees[0].len() {
            if x == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }
    // Iterations for Xs in reverse
    for y in 0..trees.len() {
        let mut current_tree_size = 0;
        for x in (0..trees[0].len()).rev() {
            if x == trees.len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    // Iterations for Ys (Up to Down and Down to Up)
    // Just switch the x and y loops
    for x in 0..trees.len() {
        let mut current_tree_size = 0;
        for y in 0..trees[0].len() {
            if y == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }
    // Iterations for Ys in reverse
    for x in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for y in (0..trees[0].len()).rev() {
            if y == trees.len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    // In this code, iter() returns an iterator over the elements, flatten() flattens the nested structure, and filter() filters the elements based on the condition |&&value| value. The && is used to dereference the reference to the bool value. Finally, count() returns the number of elements that satisfy the condition, and to_string() converts the count to a string.
    visible_trees
        .iter()
        .flatten()
        // Use && to dereference the reference to the bool
        // We have a double shared reference from iter() and filter()
        // we can just pass the boolean value back in, it's true if visible, false if not visible
        // A reference to a reference looks like this:
        .filter(|&&value| value)
        // after the filter, we have a single shared reference that looks like this:
        // &bool
        // so value represents that atual bool value. Therefore, the type of value in the clousure is &bool (from |&&value| value)
        // count() returns the number of elements that satisfy the condition: whcih is the number of visible trees (true values)
        .count()
        // convert the count to a string
        .to_string()
} // this would print out visible_trees when running the command `

pub fn process_part2(input: &str) -> String {
    let (_, trees) = parse_trees(input).unwrap();
    // initialize high_score to 0
    let mut high_score = 0;

    // get the max x and y values from the trees
    let y_max = trees.len();
    let x_max = trees[0].len();

    // iterate over the trees
    for (y_index, tree_line) in trees.iter().enumerate() {
        // for each tree (the x_index and the treehouse_height), we need to compare it to the trees around it
        for (x_index, treehouse_height) in tree_line.iter().enumerate() {
            // initialize scores to 0
            let mut scores = [0, 0, 0, 0];

            // println!(
            //     "x: {x_index}, y: {y_index}, tree height: {treehouse_height}";
            // );

            // now we compare treehouse_height to the trees around it
            // if the treehouse_height is less than the tree around it, we add 1 to the score
            // if the treehouse_height is greater than the tree around it, we add 1 to the score and break out of the loop

            // right to left
            for x_position in (0..x_index).rev() {
                // If any of the trees at the current `y_index` and `x_position` (the right to left position) are less than the `treehouse_height`, add 1 to the score. Otherwise, add 1 to the score and break out of the loop.
                if trees[y_index][x_position] < *treehouse_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }

            // left to right
            for x_position in (x_index + 1)..x_max {
                if trees[y_index][x_position] < *treehouse_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            // down to up
            for y_position in (0..y_index).rev() {
                if trees[y_position][x_index] < *treehouse_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }

            // up to down
            for y_position in (y_index + 1)..y_max {
                if trees[y_position][x_index] < *treehouse_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }
            let scenic_score: u32 = scores.iter().product();

            if scenic_score > high_score {
                high_score = scenic_score;
            }
        }
    }
    high_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "21");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "8");
    }
}
