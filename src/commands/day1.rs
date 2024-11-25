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
    pub fn good_add(&self, a: i16, b: i16) -> i16 {
        a + b
    }
}


#[cfg(test)]
mod tests {

// Note this useful idiom: importing names from outer (for mod tests) scope.
 use super::*;

 #[test]
 fn test_add() {
    let d = Day1{ message: None};
     assert_eq!(d.good_add(1, 2), 3);
 }
}