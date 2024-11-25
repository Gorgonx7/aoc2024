use anyhow::Result;
use clap::Parser;
pub mod commands;
pub mod pkg;
use crate::commands::day1::Day1;
use crate::commands::day2::Day2;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
enum CLI {
    #[clap(name = "day1")]
    Day1(Day1),
    #[clap(name = "day2")]
    Day2(Day2),
}

impl CLI {
    fn execute(&self) -> Result<()> {
        match self {
            Self::Day1(day1) => day1.execute(),
            Self::Day2(day2) => day2.execute(),
        }
    }
}

fn main() {
    let args = CLI::parse();
    match args.execute() {
        Ok(_) => println!("OK"),
        Err(err) => panic!("command failed: {}", err),
    }
}
