# Commands Used

All of these ran at the root folder (where this README.md file is located) 
`~/Developer/advent-of-code/2023/rust/day-01`

This is because at the root folder, there is a Cargo.toml file, which is the 
manifest for Rust's package manager. This file contains all the metadata for the project, such as the name, version, authors, dependencies, etc. It is crucial for building and running the Rust project.

Command | Description
--- | ---
`cargo new day-01` | Creates a new Rust project called day-01
`cargo build` | Builds the Rust project
`cargo run` | Runs the Rust project
`cargo run bin part-1` | Runs the Rust project with the `part-1` argument in the `bin` binary
`cargo run bin part-2` | Runs the Rust project with the `part-2` argument in the `bin` binary
`cargo watch -x "run --bin part-1` | Runs the Rust project with the `part-1` argument in the `bin` binary, and watches for changes in the source code
`cargo watch -x "run --bin part-2` | Runs the Rust project with the `part-2` argument in the `bin` binary, and watches for changes in the source code
`cargo test` | Runs the Rust project's tests
`mkdir bin` | Creates a new directory named `bin`
