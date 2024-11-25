# Advent of Code 2024 - Rust Edition

This is my advent of code 2024 rust edition project. I have attempted to make the boilerplate I run the code on as generalized as possible, in order to facilitate using this over more years of advent of code.

I have created a special `Template` branch, which will be updated with small util functions over time, as I find I need them, in order to make solving issues easier.

For now it just has the `utils.rs` file with the day1 and day2 commands.

## Usage
1. Add you problem data into data.txt
1. Add a new command, [see section below](#adding-a-new-day)
1. Add business logic to your new command, feel free to add tests too
1. Basic usage:
```cargo run day<number>```

## Adding a new day
Adding a new day is easy:
1. Create a new file called `day<number>.rs` to the commands directory, where the number is the day you are working on
1. Add `pub mod day<number>;` to the mod.rs in the commands directory
1. add the following boilerplate to `day<number>.rs`
```
use anyhow::{Ok, Result};
use clap::Parser;
use super::pkg;
use pkg::util::read_file;


#[derive(Parser)]
pub struct Day<number> {
}

impl Day<number> {
    pub fn execute(&self) -> Result<()> {
        let data = read_file("data.txt".to_string());
        print!("data: {}", data);
        Ok(())
    }
}
```
NOTE: Ensure you replace `<number>` with the number of the day you are working on, i.e `3`
1. In main.rs add `use create::commands::day<number>::Day<number>`, then in the `enum CLI` add: 
```
#[clap(name = "day<number>")]
Day<number>(Day<number>),
```
NOTE: again replace `<number>` with the day you are working on

1. In `fn execute` add the line: 
```
Self::Day<number>(day<number>) => day<number>.execute(),
```
1. Test you've added your new command by using: 
```
cargo run day<number>
```
This should output 
```
data: hello, world
OK
```

With the default data file.