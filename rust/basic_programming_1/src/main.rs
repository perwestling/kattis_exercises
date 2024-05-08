use std::{collections::BTreeSet, io::{self, prelude::*}};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let line1 = stdin_iter.next().expect("Missing line 1").unwrap();
    let line2 = stdin_iter.next().expect("Missing line 2").unwrap();
    let (command, numbers) = handle_input(&line1, &line2);
    println!("{}", act_on_input(command, numbers));

    ()
}

fn handle_input(line1: &str, line2: &str) -> (u32, Vec<i32>) {
    let parts1 = get_u32_numbers(line1);
    let parts2 = get_i32_numbers(line2);
    let command : u32 = parts1[1];
    (command, parts2)
}

fn get_u32_numbers(s: &str) -> Vec<u32> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<u32>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn get_i32_numbers(s: &str) -> Vec<i32> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<i32>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn act_on_input(command: u32, numbers: Vec<i32>) -> String {
    let result = match command {
        1 => "7".to_string(),
        2 => handle_2(&numbers).to_string(),
        3 => handle_3(&numbers),
        4 => handle_4(&numbers),
        5 => handle_5(&numbers),
        6 => handle_6(&numbers),
        7 => handle_7(&numbers).to_string(),
        _ => panic!("Not supported command: {}", command),
    };
    result
}

fn handle_2(numbers: &Vec<i32>) -> &str {
    if numbers[0] > numbers[1] {
        return "Bigger";
    }
    if numbers[0] < numbers[1] {
        return "Smaller";
    }
    return "Equal";
}

fn handle_3(numbers: &Vec<i32>) -> String {
    let mut first_3: Vec<i32> = numbers
        .iter()
        .take(3)
        .copied()
        .collect();
    first_3.sort();
    return first_3.get(1).unwrap().to_string();
}

fn handle_4(numbers: &Vec<i32>) -> String {
    return numbers
        .iter()
        .map( |&x| x as i64 )
        .sum::<i64>()
        .to_string();
}

fn handle_5(numbers: &Vec<i32>) -> String {
    return numbers
      .iter()
      .filter( |&&x| x % 2 == 0 )
      .map( |&x| x as i64 )
      .sum::<i64>()
      .to_string();
}

fn handle_6(numbers: &Vec<i32>) -> String {
    return numbers
      .iter()
      .map( |&x| x % 26 )
      .map( |x| ('a' as u32).wrapping_add_signed(x) as u8 as char )
      .collect::<String>();
}

fn handle_7(numbers: &Vec<i32>) -> &str {
    let mut visited: BTreeSet<usize> = BTreeSet::new();
    let size = numbers.len();
    let mut index: usize = 0;
    loop {
        visited.insert(index);
        if index >= size {
            return "Out";
        }
        if index == size - 1 {
            return "Done";
        }
        index = numbers[index] as usize;
        if visited.contains(&index) {
            return "Cyclic";
        }
    }
}

#[cfg(test)]
mod tests;
