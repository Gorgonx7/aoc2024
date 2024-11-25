use anyhow::{Ok, Result};
use clap::Parser;
pub fn hello_world() {
    println!("Hello world");
}

#[derive(Parser)]
pub struct Day1 {
    message: Option<String>,
}

impl Day1 {
    pub fn execute(&self) -> Result<()> {
        let value = self.message.as_deref().unwrap_or("world");
        println!("hello {}", value);
        Ok(())
    }
}
