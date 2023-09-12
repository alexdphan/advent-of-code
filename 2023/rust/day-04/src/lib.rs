// explain this line of code in one comment
use std::ops::{Range, RangeInclusive};
// nom is a parser combinator library, meaning it is a library that allows you to combine small parsers to create more complex parsers
// a parser is a function that takes an input and returns a result
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};
// IResult is a type alias for Result<&str, nom::error::Error<&str>>, which is basically a Result type that returns a string slice and an error
// we have a parser for one range, then one line, then a whole section
fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    // we use the nom::character::complete::u32 parser to parse a u32 from the input; complete is a module that contains parsers that consume the entire input, whereas streaming is a module that contains parsers that consume the input until they fail
    // u32 would conflict with the u32 type, so we have to specify the full path to the u32 type from the nom library instead
    // Endianess means the order in which bytes are stored in memory; little means the least significant byte is stored first. This is used for parsing binary data, which is the default for nom; we'll just use complete::u32() instead

    // This line attempts to parse a u32 integer from the beginning of input.
    // If successful, the parsed integer is stored in start, and the remaining portion of the input is stored back in input.
    // If there's an error, the containing function will return early with that error (?)
    // Parse a u32 from the input (hence u32(input)) and assign it to the variable start
    // instead of using nom::character::complete::u32(input)?;, we can use complete::u32(input)?; because we have already imported the complete module
    let (input, start) = complete::u32(input)?;
    // assign (input, start) to the result of the tag parser, which will parse a string slice that matches the string slice passed in (in this case, "-"
    let (input, _) = tag("-")(input)?;

    // parse a u32 from the input and assign it to the variable end
    // doing the same, but for the end of the range
    let (input, end) = complete::u32(input)?;
    // return the input and the range from start to end (..= means inclusive (inclusive means the range includes the start and end values)))

    // you could use separated_pair to parse a pair of ranges instead of a pair of sections (TODO later?)

    Ok((input, start..=end))
}

fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    // because we already have sections (which is a parser for a range), we can use that to parse a range for our line parser

    // let (input, start) = sections(input)?;
    // _ means that we don't care about the value of the parser result
    // let (input, _) = tag(", ")(input)?;
    // let (input, end) = sections(input)?;

    // instead of the code above, we could use separated_pair to parse a pair of ranges instead of a pair of sections
    // this is a parser combinator that takes two parsers and returns a parser that parses a pair of elements
    // arguments:
    // first The first parser to apply.
    // this is the parser that is applied first, which is sections in this case
    // sep The separator parser to apply.
    // a separator parser is a parser that parses a separator (in this case, a comma)
    // it does this because we want to parse a pair of ranges, and we want to separate the two ranges with a comma
    // second The second parser to apply.
    // this is the parser that is applied second, which is sections again in this case

    // reffering to the input, we want to parse a pair of ranges, separated by a comma
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, (start, end)))
}

// our parser will take a string slice as input and return rest of the string slice (&str) and a vector of ranges (the successful return type: IResult<&str, Vec<Range<u32>, Range<u32>)>>)
// the string slice is the input to the parser
// you can also put in the third argument, which is the error type, but we don't need to do that here
fn section_assignments(
    input: &str,
) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    // separated_list1 is a parser combinator that takes a parser and a separator parser and returns a parser that parses a list of items separated by the separator parser
    // alternates between two parsers, separated by a separator parser to produce a list of elements
    // pass in the parser that is the separator (in this case, newline) and the parser that is the element or value (in this case, lin), then pass in the input
    // it first parses the separator parser, then the element parser, then the separator parser, then the element parser, etc.
    // Parse a list of 'line' elements separated by newlines from the input.
    // separated_list1 fails if it doesn't find at least one element (whereas separated_list0 doesn't fail if it doesn't find at least one element)
    // https://docs.rs/nom/latest/nom/multi/fn.separated_list1.html
    let (input, ranges) = separated_list1(newline, line)(input)?;
    Ok((input, ranges))
}

pub fn process_part1(input: &str) -> String {
    // assignments is a vector of ranges, which is the successful return type of section_assignments
    // _ is the input, which is the string slice that is passed into section_assignments
    // _ just means that we don't care about the value of the input (we ignore it)
    let (_, assignments) = section_assignments(input).unwrap();

    // we want to filter the assignments (vector) to find the number of assignments that contain each other
    let result = assignments
        // iter() returns an iterator over the vector, which is a sequence of elements that can be iterated over
        // we use .iter() instead of .into_iter() because we want to iterate over the vector WITHOUT taking ownership of it (of which we would use .into_iter() for)
        .iter()
        // we filter the vector to find the number of assignments that contain each other
        // so here, range_a is the first range, and range_b is the second range
        // |(range_a, range_b)| is a closure that takes two arguments, range_a and range_b
        // we want to check if range_a contains range_b or if range_b contains range_a
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a
                // we clone the range because we want to iterate over the range without taking ownership of it
                // we iterate over the range because we want to check if all of the elements in the range satisfy a condition
                // in this case, the range we are iterating over is range_a
                .clone()
                // we use .into_iter() because we want to iterate over the range and take ownership of it (includes mut, &mut, and &)
                .into_iter()
                // we use .all() because we want to check if all of the elements in the range satisfy a condition
                .all(|num| range_b.contains(&num));

            let b_contains_a = range_b
                // in this case, the range we are iterating over is range_a
                .clone()
                .into_iter()
                .all(|num| range_a.contains(&num));

            // we use || because we want to check if either of the conditions are true
            // would return true if either of the conditions are true
            a_contains_b || b_contains_a
        })
        // we use .count() because we want to count the number of elements in the vector that satisfy this condition
        .count();
    // we use .to_string() because we want to convert the result to a string
    // the result is a number, which we want to represent as a string
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    "result".to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
        print!("Part 1 test works")
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
        print!("Part 2 test works")
    }
}
