use std::{collections::BTreeSet, io::{self, prelude::*}};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let line1 = stdin_iter.next().expect("Missing line 1").unwrap();
    let line2 = stdin_iter.next().expect("Missing line 2").unwrap();
    let (command, numbers) = handle_input(&line1, &line2);
    println!("{}", act_on_input(command, numbers));

    ()
}

fn handle_input(line1: &str, line2: &str) -> (u32, Vec<u32>) {
    let parts1 = get_numbers(line1);
    let parts2 = get_numbers(line2);
    let command : u32 = parts1[1];
    (command, parts2)
}

fn act_on_input(command: u32, numbers: Vec<u32>) -> String {
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

fn handle_2(numbers: &Vec<u32>) -> &str {
    if numbers[0] > numbers[1] {
        return "Bigger";
    }
    if numbers[0] < numbers[1] {
        return "Smaller";
    }
    return "Equal";
}

fn handle_3(numbers: &Vec<u32>) -> String {
    let mut first_3: Vec<u32> = numbers
        .iter()
        .take(3)
        .copied()
        .collect();
    first_3.sort();
    return first_3.get(1).unwrap().to_string();
}

fn handle_4(numbers: &Vec<u32>) -> String {
    return numbers
        .iter()
        .sum::<u32>()
        .to_string();
}

fn handle_5(numbers: &Vec<u32>) -> String {
    return numbers
      .iter()
      .filter( |&&x| x % 2 == 0 )
      .sum::<u32>()
      .to_string();
}

fn handle_6(numbers: &Vec<u32>) -> String {
    return numbers
      .iter()
      .map( |&x| x % 26 )
      .map( |x| ('a' as u32 + x) as u8 as char )
      .collect::<String>();
}

fn handle_7(numbers: &Vec<u32>) -> &str {
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

fn get_numbers(s: &str) -> Vec<u32> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<u32>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_input_for_sample_1() {
        assert_eq!((1, vec![1, 2, 3, 4, 5, 6, 7]), handle_input("7 1", "1 2 3 4 5 6 7"));
    }

    #[test]
    fn test_act_on_input_command_1() {
        assert_eq!("7", act_on_input(1, vec![1, 2, 3]));
    }

    #[test]
    fn test_act_on_input_command_2() {
        assert_eq!("Bigger", act_on_input(2, vec![3, 2, 3]));
        assert_eq!("Equal", act_on_input(2, vec![2, 2, 2]));
        assert_eq!("Smaller", act_on_input(2, vec![1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn test_act_on_input_command_3() {
        assert_eq!("2", act_on_input(3, vec![1, 2, 3, 4, 5, 6, 7]));
        assert_eq!("5", act_on_input(3, vec![7, 2, 5]));
        assert_eq!("0", act_on_input(3, vec![0, 0, 1]));
    }

    #[test]
    fn test_act_on_input_command_4() {
        assert_eq!("15", act_on_input(4, vec![1, 2, 3, 4, 5]));
        assert_eq!("15", act_on_input(4, vec![5, 4, 3, 2, 1]));
        assert_eq!("28", act_on_input(4, vec![1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn test_act_on_input_command_5() {
        assert_eq!("0", act_on_input(5, vec![1]));
        assert_eq!("6", act_on_input(5, vec![1, 2, 3, 4]));
        assert_eq!("12", act_on_input(5, vec![1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn test_act_on_input_command_6() {
        assert_eq!("abc", act_on_input(6, vec![0, 1, 2]));
        assert_eq!("helloworld", act_on_input(6, vec![7, 4, 11, 37, 14, 22, 40, 17, 11, 3]));
    }

    #[test]
    fn test_act_on_input_command_7() {
        assert_eq!("Out", act_on_input(7, vec![1, 14, 3]));
        assert_eq!("Done", act_on_input(7, vec![1, 14]));
        assert_eq!("Cyclic", act_on_input(7, vec![1, 0, 2]));
    }
}

