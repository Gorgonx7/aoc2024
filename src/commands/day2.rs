use anyhow::{Ok, Result};
use clap::Parser;
use super::pkg;
use pkg::util::read_file;


#[derive(Parser)]
pub struct Day2 {
}

impl Day2 {
    pub fn execute(&self) -> Result<()> {
        let data = read_file("data.txt".to_string());
        print!("data: {}", data);
        Ok(())
    }
}
