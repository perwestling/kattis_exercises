use std::{collections::BTreeSet, collections::BTreeMap, io::{self, prelude::*}};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let first_line = stdin_iter.next().expect("Missing line 1").unwrap();
    let number_line = stdin_iter.next().expect("Missing line 2").unwrap();
    let (n, t) = handle_input(&first_line);
    println!("{}", act_on_input(t, n, &number_line));

    ()
}

fn handle_input(first_line: &str) -> (u32, u32) {
    let parts = get_u32_numbers(first_line);
    let n : u32 = parts[0];
    let t : u32 = parts[1];
    (n, t)
}

fn get_u32_numbers(s: &str) -> Vec<u32> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<u32>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn act_on_input(t: u32, n: u32, number_line: &str) -> String {
    let result = match t {
        1 => handle_1(number_line).to_string(),
        2 => handle_2(number_line).to_string(),
        3 => handle_3(n, number_line),
        4 => handle_4(n, number_line),
        5 => handle_5(number_line),
        _ => panic!("Not supported command: {}", t),
    };
    result
}

fn handle_1(number_line: &str) -> &str {
    let mut numbers : BTreeSet<u32> = BTreeSet::new();
    for item in number_line.to_string().split_whitespace() {
        let x = item.to_string().parse::<u32>().unwrap();
        if x <= 7777 {
            let y = 7777 - x;
            if numbers.contains(&y) {
                return "Yes";
            }
            numbers.insert(x);
        }
    }
    return "No";
}

fn handle_2(number_line: &str) -> &str {
    let mut numbers : BTreeMap<u32, ()> = BTreeMap::new();
    for item in number_line.to_string().split_whitespace() {
        let val = item.to_string().parse::<u32>().unwrap();
        match numbers.insert(val, ()) {
            None => {},
            Some(_) => { return "Contains duplicate"; },
        }
    }
    return "Unique";
}

fn handle_3(count: u32, number_line: &str) -> String {
    let threshold = (count / 2) as u64;
    let mut numbers : BTreeMap<u32, u64> = BTreeMap::new();
    for item in number_line.to_string().split_whitespace() {
        let val = item.to_string().parse::<u32>().unwrap();
        match numbers.get(&val) {
            Some(&y) => numbers.insert(val, y + 1),
            None => numbers.insert(val, 1 as u64),
        };
    }
    for (&key, &value) in numbers.iter() {
        if value > threshold {
            return key.to_string();
        }
    }
    return "-1".to_string();
}

fn handle_4(count: u32, number_line: &str) -> String {
    let mut numbers = get_u32_numbers(number_line);
    numbers.sort();
    if count % 2 == 0 { // Even
        let higher_median : usize = (count / 2) as usize;
        let lower_median : usize = higher_median - 1;
        return format!("{} {}", numbers[lower_median], numbers[higher_median]);
    }
    else { // Odd
        let median : usize = (count / 2) as usize;
        return numbers[median].to_string();
    }
}

fn handle_5(number_line: &str) -> String {
    let mut numbers : Vec<u32> = Vec::new();
    for item in number_line.to_string().split_whitespace() {
        let val = item.to_string().parse::<u32>().unwrap();
        if val < 1000 && 100 <= val {
            numbers.push(val);
        }
    }
    numbers.sort();
    let result: String = numbers.iter()
                                .map(|num| num.to_string())
                                .collect::<Vec<String>>()
                                .join(" ");
    return result;
}

#[cfg(test)]
mod tests;
