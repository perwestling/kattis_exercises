use std::collections::VecDeque;
use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
    for line in io::stdin().lock().lines() {
        println!("{}", remove_less_than(line.unwrap()));
    }
    Ok(())
}

fn remove_less_than(input: String) -> String {
    let mut result: VecDeque<char> = VecDeque::new();
    for c in input.chars() {
        if c == '<' {
            result.pop_back();
            continue;
        }
        result.push_back(c);
    }
    return result.into_iter().collect();
}

#[cfg(test)]
mod tests;
