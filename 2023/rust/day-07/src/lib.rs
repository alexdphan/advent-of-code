// the <'a> is a lifetime specifier used in structs and enums for references, it's saying that the reference will live for the lifetime 'a (in this case, the lifetime of the struct or enum would be used here for 'a)

// #![feature(iter_intersperse)]
use std::collections::BTreeMap;

use nom::{
    // alt() matches its input against a list of parsers, and returns the result of the first one that succeeds.
    branch::alt,
    // is_a() matches one or more characters from the provided string.
    // tag() matches a specific string
    bytes::complete::{is_a, tag},
    // alpha1() matches one or more alphabetic characters, which means that it will match a, ab, abc, etc.
    // newline() matches a newline character
    character::complete::{alpha1, newline},
    // separated_list1() matches one or more occurrences of the first parser separated by the second parser, which means that it will match the first parser, then the second parser, then the first parser, then the second parser, etc.
    multi::separated_list1,
    // separated_pair() takes three parsers: the first parser, a separator parser, and the second parser.
    // It matches the first parser, then the separator (discarding the separator's result), and finally the second parser.
    // It returns the results of the first and second parsers in a tuple.
    // Ex: separated_pair(tag("a"), tag("b"), tag("c")) matches "abc" and returns ("a", "c").
    sequence::separated_pair,
    // IResult is a type alias for Result<&str, nom::error::Error<&str>>
    IResult,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

// 8504156 c.dat
// dir d
// 29116 f
// 2557 g
// 62596 h.lst
// function that takes in input, returns a tuple of (&str, Files)
fn file(input: &str) -> IResult<&str, Files> {
    // separated_pair: Gets an object from the first parser, then matches an object from the sep_parser and discards it, then gets another object from the second parser.
    // the separated_pair is used to match the size and the name of the file
    // in this case, the separated_pair discards the space between the size and the name of the file
    let (input, (size, name)) = separated_pair(
        // nom::character::complete::u32: matches a u32 which is a number
        // this would match something like "8504156", which is the size of the file
        nom::character::complete::u32,
        // tag: matches a specific string, in this case, a space
        // then because of separated_pair, it matches a space
        tag(" "),
        // is_a: matches a string of characters, in this case, a string of characters that are either a-z, A-Z, 0-9, or "."
        // this would match something like "c.dat", which is the name of the file
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;
    // Ok means that the function was successful, and returns the input and the Files::File { size, name }
    Ok((input, Files::File { size, name }))
} // output would be something like (input, Files::File { size: 8504156, name: "c.dat" })

// dir a
// dir d
// dir e
// function that takes in input, returns a tuple of (&str, Files)
// <&str, Files> means it returns a tuple of a string slice and a Files enum
// <> is a generic type, which means that it can be used for any type
fn directory(input: &str) -> IResult<&str, Files> {
    // tag: matches a specific string, in this case, "dir "
    // uses _ to ignore the output of tag("dir ")
    let (input, _) = tag("dir ")(input)?;
    // takes in input, returns a name (Vec<char>)
    // alpha1: matches one or more alphabetic characters (a-z, A-Z) for the name of the directory which is a string
    // (input, name) is a tuple of (&str, &str) that is destructured into input and name (the former is the remaining input, the latter is the name of the directory (the matched value))
    let (input, name) = alpha1(input)?;
    // if Ok (successful), then return the input and Files::Dir(name)
    Ok((input, Files::Dir(name)))
} // output would be something like (input, Files::Dir("a"))

// $ ls
fn ls(input: &str) -> IResult<&str, Operation> {
    // finds the tag "$ ls" in the input
    let (input, _) = tag("$ ls")(input)?;
    // newline is here because the input is $ ls\n
    // Consumes the newline character after "$ ls" and discards it
    // ex: $ ls\n
    // newline is a parser that matches a newline character from nom::character::complete::newline
    // discards the result of newline
    let (input, _) = newline(input)?;
    // takes in input, returns files (Vec<Files>)
    // uses separated_list1 to alternate between newline and either file or directory
    // ex: $ ls\n, dir a\n, 14848514 b.txt\n, 8504156 c.dat\n, dir d\n
    // basically does new line, then shows either a file or directory, then new line, then shows either a file or directory, etc. until it reaches the end of the input
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    // if Ok (successful), then return the input and Operation::Ls(files)
    Ok((input, Operation::Ls(files)))
} // output would be something like (input, Operation::Ls([Files::Dir("a"), Files::File { size: 14848514, name: "b.txt" }, Files::File { size: 8504156, name: "c.dat" }, Files::Dir("d")]))

// $ cd a
// $ cd e
// $ cd d
// $ cd ..
fn cd(input: &str) -> IResult<&str, Operation> {
    // finds the tag "$ cd " in the input and discards it
    let (input, _) = tag("$ cd ")(input)?;
    // takes in input, returns either ".." or a directory name (using alt to match either tag("..") or alpha1 or tag("/")))
    // returns ..  because the input is $ cd ..\n
    // returns a directory name because the input is $ cd a\n
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    // op means operation to match either Cd::Root, Cd::Up, or Cd::Down(name) using "/", "..", or name
    let op = match dir {
        // if "/", then go to Root
        "/" => Operation::Cd(Cd::Root),
        // if "..", then go Up
        ".." => Operation::Cd(Cd::Up),
        // otherwise for any other directory name, go Down
        name => Operation::Cd(Cd::Down(name)),
    };
    // if Ok (successful), then return the input and op
    Ok((input, op))
} // output would be something like (input, Operation::Cd(Cd::Down("a")))

// takes in input, returns a tuple of (&str, Vec<Operation>)
// &str refers to the input, Vec<Operation> refers to the commands
// the input refers to the input from the user, which is a string
fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    // takes in input, returns the cmd (Vec<Operation>)
    // uses separated_list1 to alternate between newline and either ls or cd
    // ex: $ cd /\n, $ ls\n, dir a\n, 14848514 b.txt\n, 8504156 c.dat\n, dir d\n, $ cd a\n, $ ls\n, dir e\n, 29116 f\n, 2557 g\n, 62596 h.lst\n, $ cd e\n, $ ls\n, 584 i\n, $ cd ..\n, $ cd ..\n, $ cd d\n, $ ls\n, 4060174 j\n, 8033020 d.log\n, 5626152 d.ext\n, 7214296 k
    // Return the input and cmd (which alternates between ls and cd and is a Vec<Operation>) if successful
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    // if Ok (successful), then return the input and cmd
    Ok((input, cmd))
} // there is no output because it returns the input and cmd, which is a Vec<Operation> that contains either Ls(files) or Cd(name)
  // so it returns the input (which is the remaining input) and the commands (which is a Vec<Operation> that contains either Ls(files) or Cd(name)
  // this is where we parse the input to get the commands

// function that takes in a tuple of (Vec<&str>, BTreeMap<Vec<&str>, u32>) and an Operation, returns a tuple of (Vec<&str>, BTreeMap<Vec<&str>, u32>)
// Vec<T> and BTreeMap<K, V> are generic definitions where T, K, and V are type parameters that can be replaced with concrete types when you use these structures.
// Vec<T> and BTreeMap<K, V> but the types are specified as Vec<&str> and BTreeMap<Vec<&str>, u32>
// BTreeMap is a map based on a binary tree, which means that the keys are sorted by their order in the tree (order is determined by the Ord trait)
// ---------- Parameters ----------
// defines context as a Vec<&str> and sizes as a BTreeMap<Vec<&str>, u32> (both are mutable parameters). This is the first parameter of the function.
// command: &'a Operation is a reference to an Operation which is the command (this is an immutable parameter). This is the second parameter of the function.
fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    command: &'a Operation,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        // Navigate to the root directory.
        Operation::Cd(Cd::Root) => {
            // push() adds an element to the end of the vector
            // The push() method doesn't "return" the updated vector; instead, it modifies the vector in-place.
            // For example: If context is vec![""], after push("") it becomes vec!["", ""]
            context.push("");
        }
        // Navigate up to the parent directory.
        Operation::Cd(Cd::Up) => {
            // pop() removes the last element from the vector and returns it
            // The pop() method returns an Option<T> because the vector might be empty.
            // For example: If context is vec!["", ""], after pop() it becomes vec![""]
            context.pop();
        }
        // Navigate down to a specified child directory.
        Operation::Cd(Cd::Down(name)) => {
            // push() adds an element to the end of the vector
            // The push() method doesn't "return" the updated vector; instead, it modifies the vector in-place.
            // For example: If context is vec![""], after push("a") it becomes vec!["", "a"]
            context.push(name);
        }
        // List the files in the current directory and update their sizes.
        Operation::Ls(files) => {
            // Calculate the total size of all files in the current directory.
            let sum = files
                // iter() creates an iterator over the vector
                .iter()
                // filter_map() creates an iterator that both filters and maps the values, which means that it returns an iterator that applies a function to each element and only returns the elements for which the function returns Some(value).
                .filter_map(|file| {
                    if let Files::File { size, .. } = file {
                        // If the file is a file, return Some(size).
                        Some(size)
                    } else {
                        // If the file is a directory, return None.
                        None
                    }
                })
                // sum() returns the sum of all elements in the iterator (which is the total size of all files in the current directory, which is the vector)
                .sum::<u32>();

            // Update the sizes map for all segments of the current path.
            // for example, if you have directory b inside directory a, then you would have ["", "a", "b"] for the context because you would have to go to the root directory, then go to directory a, then go to directory b
            // going from 0 to the length of the context (which is the number of directories in the context)
            for i in 0..context.len() {
                // sizes is a BTreeMap<Vec<&str>, u32> that we will use in this fold() function
                sizes
                    // entry() returns an Entry which is an enum that represents a value that might or might not exist in the map
                    // Gets the given key's corresponding entry in the map for in-place manipulation.
                    .entry(context[0..=i].to_vec())
                    // and_modify() modifies an existing entry
                    // 0 to 0 would be the root directory
                    // 0 to 1 would be the root directory plus the first directory
                    // 0 to 2 would be the root directory plus the first directory plus the second directory, etc.
                    .and_modify(|v| *v += sum)
                    // or_insert() inserts a new entry if the key doesn't exist
                    .or_insert(sum);
            }
        }
    };
    // Return the updated context and sizes.
    (context, sizes)
}
// output would be something like (context, sizes)
// the output is not the sum of the sizes, but rather a mapping of directory contexts to their respective sizes.
// ex: (["", "", "a"], {["", "", "a"]: 100, ["", ""]: 100, [""]: 100})

pub fn process_part1(input: &str) -> String {
    // get's the commands from the input and stores it in cmds, which is a Vec<Operation> that contains either Ls(files) or Cd(name)
    // .unwrap().1 means that it returns the second element of the tuple, which is for the commands
    // parse the input to get the commands, then store it in cmds
    // .unwrap().1 means that it returns the second element of the tuple, which contains the commands
    // Given that the tuple is of the form (&str, Vec<Operation>) (from the commands(), extracting with .1 gives you the Vec<Operation>, which is a list of parsed commands.
    let cmds = commands(input).unwrap().1;

    // let (_, sizes) means that it returns the second element of the tuple, which is for the sizes
    // we iterate over the commands and calculate the sizes for each command
    // .fold(): fold() takes two arguments: an initial value, and a closure with two arguments: an 'accumulator', and an element. The closure returns the value that the accumulator should have for the next iteration.
    // in this case, the initial value is (vec![], BTreeMap::new())
    // the closure is calculate_sizes.
    // The closure (calculate_sizes) takes in two arguments: an accumulator and an element.
    // The accumulator is a tuple of (Vec<&str>, BTreeMap<Vec<&str>, u32>)
    // The element is an Operation, which is either Ls(files) or Cd(name)
    // The initial value is the value the accumulator will have on the first call.
    // After applying this closure to every element of the iterator, fold() returns the accumulator.
    // This operation is sometimes called ‘reduce’ or ‘inject’.
    // Folding is useful whenever you have a collection of something, and want to produce a single value from it.
    // .fold((vec![], BTreeMap::new()), calculate_sizes) means that it returns a tuple of (Vec<&str>, BTreeMap<Vec<&str>, u32>)
    // calculate_sizes() takes in a tuple of (Vec<&str>, BTreeMap<Vec<&str>, u32>) and an Operation, returns a tuple of (Vec<&str>, BTreeMap<Vec<&str>, u32>)
    // BTreeMap::new() creates a new BTreeMap in which we can store the sizes for each command
    // Calculate the sizes for each command and store them in the sizes map
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);
    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let cmds = commands(input).unwrap().1;

    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    // &vec![""] is a reference to a vector that contains an empty string
    // ! means that it's a macro that creates a vector (comes from package std)
    // used_space is the amount of space used by the empty string (which is 0)
    // the &vec![""] is a reference to a vector that contains an empty string
    // we unwrap the value of the empty string because we know that it exists
    // the vector is being referenced from the sizes map, which is a BTreeMap<Vec<&str>, u32> and contains the sizes for each command
    // This line of code retrieves the value associated with the key &vec![""] from the sizes map (in the process_part1 function)
    // The & symbol is used to create a reference to a vector that contains an empty string.
    // The get() method is called on the sizes map to retrieve the value corresponding to the key &vec![""].
    // The unwrap() method is then used to extract the value from the Option type returned by get().
    // In this case, the value represents the amount of used space, which is the size of the empty string.
    let used_space = sizes.get(&vec![""]).unwrap();

    // this would be 70_000_000 - used_space (which would be determined by the sizes map in the function calculate_sizes)
    let current_free_space = total_size - used_space;
    // this would be 30_000_000 - 70_000_000 = -40_000_000
    let need_to_free_at_least = needed_space - current_free_space;

    // sizes is a BTreeMap<Vec<&str>, u32>
    let mut valid_dirs = sizes
        // iter() creates an iterator over the vector
        .iter()
        // filter() creates an iterator that filters the values, which means that it returns an iterator that only returns the elements for which the function returns true.
        // the _ is a placeholder for the key, which is a Vec<&str> that would be discarded
        // The || symbol in Rust is used to define a closure, which is an anonymous function that can be stored in a variable or passed as an argument to other functions.
        // The closure takes no arguments (||) and returns the value size (whcih is referenced by the & symbol to get the value from the sizes map)
        .filter(|(_, &size)| size > need_to_free_at_least)
        // map() creates an iterator that maps the values, which means that it returns an iterator that applies a function to each element.
        // the || means it's a closure that takes in the input and returns the value size for each element which is the size of the directory
        .map(|(_, size)| size)
        // collect() creates a collection from the iterator that collects the values into a vector which is a vector of u32 that is referenced by the & symbol from the map() function
        .collect::<Vec<&u32>>();

    valid_dirs.sort();
    valid_dirs.iter().next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "95437");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "24933642");
    }
}
