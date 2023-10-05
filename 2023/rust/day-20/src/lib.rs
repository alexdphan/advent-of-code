use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete,
    character::complete::line_ending,
    combinator::{eof, iterator},
    multi::separated_list1,
    sequence::terminated,
    *,
};
// tracing is a crate that allows us to do logging in a structured way
use tracing::*;
// tracing_subscriber is a crate that allows us to configure how tracing works
use tracing_subscriber;

#[instrument(skip(input))]
// using i64 because we need to be able to represent negative numbers
fn numbers(input: &str) -> IResult<&str, Vec<(usize, i64)>> {
    // Creates an iterator from input data and a parser
    // we do it with an input and a parser of terminated()
    // terminated() is a combinator that runs a parser and then another one, and returns the result of the first one
    // eof() returns its input if it is at the end of input data; When we're at the end of the data, this combinator will succeed
    let mut it = iterator(input, terminated(complete::i64, alt((line_ending, eof))));

    // .enumerate() Creates an iterator which gives the current index as well as the element.
    let numbers = it.enumerate().collect::<Vec<_>>();
    // info!(?numbers) is a macro that logs the value of numbers. we need ? because numbers is a Vec<_> and we need to implement Debug for it
    info!(?numbers);
    // .finish() Returns the remaining input if parsing was successful, or the error if we encountered an error.
    let (input, _) = it.finish()?;
    Ok((input, numbers))
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> String {
    let (_, numbers) = numbers(input).unwrap();
    // we clone numbers because we need to be able to mutate it
    let mut state = numbers.clone();
    // we get the state of the numbers, which is a vector of tuples of (index, value)
    info!(?state);
    // For each id and value in numbers, we log the value
    // Then we find the position in 'state' where the first element of the tuple (the id) matches the current 'id' from 'numbers'
    // We assign this position to 'index' and unwrap it to get the value from the Option returned by 'position()'
    // this assumes no duplicate ids
    for (id, value) in numbers.iter() {
        info!(?value, "moving");
        let index = state
            .iter()
            .position(|state_value| state_value.0 == *id)
            .unwrap();

        // we remove the value at the index to get the current value
        let current = state.remove(index);
        // we assign added as the index + the current value
        // we use .1 instead of .0 because we want the value, not the index
        let added = index as i64 + current.1;
        // we get the new index by doing a modulo of the length of the state (Calculates the least nonnegative remainder of self (mod rhs).)
        // The .rem_euclid() function in Rust calculates the least nonnegative remainder of self (mod rhs). This is equivalent to the % operator in many languages, but it always returns a positive number, even when one of the operands is negative.
        let new_index = added.rem_euclid(state.len() as i64);

        // we log the index and the new index
        info!(index, new_index);

        // we insert the current value at the new index
        state.insert(new_index as usize, current);

        // we log the state
        info!("{:?}", state.iter().map(|v| v.1).collect::<Vec<_>>());
    }
    // assign zero_pos as the position of the value 0 in the state
    let zero_pos = state.iter().position(|v| v.1 == 0).unwrap();
    // assign a, b, and c as the values at the positions 1000, 2000, and 3000 from the zero_pos
    // we add 1000, 2000, and 3000 to zero_pos to get the positions
    // we use % to get the remainder of the division by the length of the state; which we use because we want to wrap around the state
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;
    // we log a, b, c, and "ABC", the values we need to return which are the values at the positions 1000, 2000, and 3000 from the zero_pos
    info!(a, b, c, "ABC");
    // we return the sum of a, b, and c as a string
    (a + b + c).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut numbers) = numbers(input).unwrap();

    // need to multiply the values by 811589153 for part 2
    numbers.iter_mut().for_each(|tuple| tuple.1 *= 811589153);

    // we clone numbers because we need to be able to mutate it
    let mut state = numbers.clone();
    // we get the state of the numbers, which is a vector of tuples of (index, value)
    info!(?state);
    // For each id and value in numbers, we log the value
    // Then we find the position in 'state' where the first element of the tuple (the id) matches the current 'id' from 'numbers'
    // We assign this position to 'index' and unwrap it to get the value from the Option returned by 'position()'
    // this assumes no duplicate ids
    for _ in 0..10 {
        for (id, value) in numbers.iter() {
            info!(?value, "moving");
            let index = state
                .iter()
                .position(|state_value| state_value.0 == *id)
                .unwrap();

            // we remove the value at the index to get the current value
            let current = state.remove(index);
            // we assign added as the index + the current value
            // we use .1 instead of .0 because we want the value, not the index
            let added = index as i64 + current.1;
            // we get the new index by doing a modulo of the length of the state (Calculates the least nonnegative remainder of self (mod rhs).)
            // The .rem_euclid() function in Rust calculates the least nonnegative remainder of self (mod rhs). This is equivalent to the % operator in many languages, but it always returns a positive number, even when one of the operands is negative.
            let new_index = added.rem_euclid(state.len() as i64);

            // we log the index and the new index
            info!(index, new_index);

            // we insert the current value at the new index
            state.insert(new_index as usize, current);

            // we log the state
            info!("{:?}", state.iter().map(|v| v.1).collect::<Vec<_>>());
        }
    }
    // assign zero_pos as the position of the value 0 in the state
    let zero_pos = state.iter().position(|v| v.1 == 0).unwrap();
    // assign a, b, and c as the values at the positions 1000, 2000, and 3000 from the zero_pos
    // we add 1000, 2000, and 3000 to zero_pos to get the positions
    // we use % to get the remainder of the division by the length of the state; which we use because we want to wrap around the state
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;
    // we log a, b, c, and "ABC", the values we need to return which are the values at the positions 1000, 2000, and 3000 from the zero_pos
    info!(a, b, c, "ABC");
    // we return the sum of a, b, and c as a string
    (a + b + c).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    // we use ignore here becuase you can instantiate with a global subscriber only once (talking here about tracing_subscriber::fmt::init();
    #[test]
    #[ignore]
    fn part1_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part1(INPUT), "3");
    }

    #[test]
    fn part2_works() {
        tracing_subscriber::fmt::init();
        assert_eq!(process_part2(INPUT), "1623178306");
    }
}

// try RUST_LOG="" cargo run --bin part-1 or RUST_LOG="" cargo run --bin part-2 to run the code without logging