use anyhow::{Ok, Result};
use clap::Parser;

#[derive(Parser)]
pub struct Day2 {
    message: Option<String>,
}

impl Day2 {
    pub fn execute(&self) -> Result<()> {
        let value = self.message.as_deref().unwrap_or("world");
        println!("day 2 hello {}", value);
        Ok(())
    }
}
