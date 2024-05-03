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
mod tests {
    use super::*;

    #[test]
    fn test_no_effect_if_none() {
        let str = String::from("Hej hopp");
        assert_eq!(str.to_string(), remove_less_than(str));
    }

    #[test]
    fn test_remove_single_charactger() {
        let str = String::from("a<");
        assert_eq!("", remove_less_than(str));
    }

    #[test]
    fn test_sample1() {
        let str = String::from("a<bc<");
        assert_eq!("b", remove_less_than(str));
    }

    #[test]
    fn test_sample2() {
        let str = String::from("foss<<rritun");
        assert_eq!("forritun", remove_less_than(str));
    }

    #[test]
    fn test_sample3() {
        let str = String::from("a<a<a<aa<<");
        assert_eq!("", remove_less_than(str));
    }

    #[test]
    fn test_abnormal1() {
        let str = String::from("<<<");
        assert_eq!("", remove_less_than(str));
    }

    #[test]
    fn test_abnormal2() {
        let str = String::from("ab<<<");
        assert_eq!("", remove_less_than(str));
    }
}

