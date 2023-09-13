use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    *,
};

// first we parse each crate
fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    // parse the tag, which would return None if it's 3 spaces
    // using alt() to parse either 3 spaces or the crate name
    let (input, c) = alt((
        tag("   "),
        // parse the crate name
        // delimited: Matches an object from the first parser and discards it, then gets an object from the second parser, and finally matches an object from the third parser and discards it.
        // remo
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    // return the result, and the remaining input
    // the remaining input is used to parse the next crate
    Ok((input, result))
}

// then we parse each line
// takes in a string, and returns a vector of crates, which is a vector of options of strings
fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    // Alternates between two parsers to produce a list of elements until Err::Error.
    // Fails if the element parser does not produce at least one element.
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

// using #[derive(Debug)] to implement the Debug trait for the Move struct
#[derive(Debug)]
struct Move {
    number: u32,
    from: u32,
    to: u32,
}

// then we parse each move instruction
fn move_crate(input: &str) -> IResult<&str, Move> {
    // use the tag parser to parse the string "move "
    let (input, _) = tag("move ")(input)?;
    // use the u32 parser to parse the number
    let (input, number) = complete::u32(input)?;
    // use the tag parser to parse the string " from "
    let (input, _) = tag(" from ")(input)?;
    // use the u32 parser to parse the number
    let (input, from) = complete::u32(input)?;
    // use the tag parser to parse the string " to "
    let (input, _) = tag(" to ")(input)?;
    // use the u32 parser to parse the number
    let (input, to) = complete::u32(input)?;
    // Ok: Returns the provided value and remaining input. Ok is used to indicate a successful parse.
    Ok((
        input,
        // from -1 and to -1 because the index starts from 0
        Move {
            number,
            from: from - 1,
            to: to - 1,
        },
    ))
}

// then we parse the whole input of crates and moves
// takes in a string, returns the remaining input, and a tuple of vectors of vectors of strings (which is the crates), and a vector of moves
// Vec<Vec<&str>> is a vector of vectors of strings, which is the crates
fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    // parse the crates which is a vector of vectors of strings
    // separated_list1: Alternates between newline and the line parser to produce a list of elements until Err::Error.
    let (input, crates_horizontal) = separated_list1(newline, line)(input)?;

    // parse the newline and discard it
    let (input, _) = newline(input)?;

    // Runs the embedded parser, gathering the results in a Vec.
    // preceded: Matches the first parser, discards its result, and runs the second parser.
    // space1: Matches one or more whitespace characters.
    // digit1: Matches one or more digits.
    // only puts a newline between the lines that we parsed
    // result is a vector of numbers
    let (input, _numbers) = many1(preceded(space1, digit1))(input)?;

    // parse the newline and discard it
    // Recognizes one or more spaces, tabs, carriage returns and line feeds.
    let (input, _) = multispace1(input)?;

    // separated_list1: Alternates between newline and the move_crate parser to produce a list of elements until Err::Error.
    // in this case, we alternate between newline and move_crate
    let (input, moves) = separated_list1(newline, move_crate)(input)?;

    // crates_vertical is a vector of vectors of options of strings (which is the crates)
    // it's assigned to an empty vector of vectors of options of strings
    // crates_vertical represents the crates in the vertical order, of which we use to get the final order of the crates
    let mut crates_vertical: Vec<Vec<Option<&str>>> = vec![];
    // for each vector of options of strings in crates_horizontal, we push an empty vector of options of strings to crates_vertical
    // this is so that we can push the crates in crates_horizontal to crates_vertical in the correct order, of which we can get the final order of the crates
    // for example, if there are several crate_horizontal vectors, we push several empty crate_vertical vectors
    // example:
    // crates_horizontal = [[A, B, C], [D, E, F], [G, H, I]]
    // crates_vertical = [[], [], []]
    // crates_vertical = [[A, D, G], [B, E, H], [C, F, I]]
    for _ in 0..=crates_horizontal.len() {
        crates_vertical.push(vec![]);
    }
    // for each vector of options of strings in crates_horizontal, we iterate through it in reverse order
    for vec in crates_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            crates_vertical[i].push(c.clone())
        }
    }
    // final_crates is a vector of vectors of strings (which is the crates)
    let final_crates: Vec<Vec<&str>> = crates_vertical
        // .iter() iterates through the vector of vectors of options of strings in crates_vertical
        .iter()
        // |vec| is a parameter of the closure passed to the map function. It represents each element of the iterator obtained by calling iter() on the vector crates_vertical. The closure is applied to each element of crates_vertical and performs operations on vec.
        // In the code snippet you provided, the v is referencing each element of the iterator v that is obtained by calling iter() on the vector crate_stacks.
        // Specifically, in the line .map(|v| match v.iter().last() { ... }), the closure |v| ... is applied to each element v of the iterator. The v represents a vector of strings obtained from crate_stacks. The closure then uses v.iter().last() to get the last element of the vector v.
        // filter_map: The returned iterator yields only the values for which the supplied closure returns Some(value). This is using v.iter().filter_map(|v| *v) to filter out the None values.
        // (v is a reference to the vector of strings, and *v is the vector of strings) Example:
        // let a = vec![Some(1), None, Some(2)];
        // let b: Vec<_> = a.iter().filter_map(|v| *v).collect();
        // assert_eq!(b, vec![1, 2]);
        .map(|vec| vec.iter().filter_map(|v| *v).collect())
        // then we collect the result into a vector of strings
        .collect();

    // return the remaining input, and a tuple of final_crates and moves
    Ok((input, (final_crates, moves)))
}

// process_part1 takes in a string, and returns a string
pub fn process_part1(input: &str) -> String {
    // _ is the remaining input
    // (mut crate_stacks, moves) is the tuple of vectors of vectors of strings (which is the crates), and a vector of moves
    // we do this because we want to modify the crate_stacks
    // overall, (mut crates_stacks, moves) is the result from the input _ (remaining input)
    // we discard the remaining input because we don't need it, but we do need to modify the crate_stacks and output moves
    let (_, (mut crate_stacks, moves)) = crates(input).unwrap();
    // results in an input of crate_stacks and output of moves

    // for Move { number, from, to } in moves.iter() {, we iterate through the moves and destructure it
    // iterate through all of the moves we need to apply
    for Move { number, from, to } in moves.iter() {
        // we set len to the length of the vector of strings in crate_stacks[*from as usize]
        let len = crate_stacks[*from as usize].len();
        // draining crate_stacks off into a vector of strings sliced from the top of the stack
        let drained = crate_stacks[*from as usize]
            // drain: Removes the specified range in the vector, and returns the removed items as a drain iterator.
            // in this case, we remove the range (len - *number as usize).. from the vector of strings in crate_stacks[*from as usize]
            // rev: Reverses an iterator's direction.
            // we do this because we want to remove the crates from the top of the stack
            // then we get the length of the vector of strings in crate_stacks[*from as usize] (with .collect::<Vec<&str>>())
            // * is the dereference operator, which dereferences the pointer, meaning that we get the value of the pointer
            // example:
            // let a = 1;
            // let b = &a;
            // let c = *b;
            // println!("{}", c); // 1
            .drain((len - *number as usize)..)
            // we do this in reverse order because we want to remove the crates from the top of the stack
            .rev()
            .collect::<Vec<&str>>();
        // for c in drained.iter() {, we iterate through the crates in drained
        // drained is a vector of strings (which is the crates)
        // we push the crates in drained to the vector of strings in crate_stacks[*to as usize]
        for c in drained.iter() {
            // indexes are always specified as usize
            // .push(c) pushes the crate to the vector of strings in crate_stacks[*to as usize]
            // we use push to push the crates to the vector of strings in crate_stacks[*to as usize] (which is the number of the stack we want to push the crates to)
            crate_stacks[*to as usize].push(c);
        }
    }

    // assign result to a type String which is set to the last crate in each stack
    let result: String = crate_stacks
        // .iter() iterates through the vector of vectors of strings in crate_stacks
        .iter()
        // .map(|v| match v.iter().last() {, we iterate through the vector of strings in crate_stacks
        .map(|v| match v.iter().last() {
            // Some(c) => c, if the crate is Some, we return the crate
            // In Rust, &&str is a reference to a reference to a string slice. The first & is the reference to the vector of strings, and the second & is the reference to the string slice.
            Some(c) => c,
            // None => "", if the crate is None, we return an empty string
            None => "",
        })
        // .collect(); collects the result into a String
        .collect();

    // return the result
    result
}

// do the same thing as process_part1, but we don't reverse the order of the crates when we move them
pub fn process_part2(input: &str) -> String {
    let (_, (mut crate_stacks, moves)) = crates(input).unwrap();
    for Move { number, from, to } in moves.iter() {
        let len = crate_stacks[*from as usize].len();
        let drained = crate_stacks[*from as usize]
            .drain((len - *number as usize)..)
            // removed .rev()
            // .rev()
            .collect::<Vec<&str>>();
        for c in drained.iter() {
            crate_stacks[*to as usize].push(c);
        }
    }
    let result: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
