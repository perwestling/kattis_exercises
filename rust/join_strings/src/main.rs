use std::{io, io::BufRead};

#[allow(unused)]
pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let mut strings : Vec<String> = Vec::new();
    let mut indexes : Vec<Vec<usize>> = Vec::new();
    let mut final_line : usize = 0;

    let possible_line = stdin_iter.next();
    let count_line = possible_line.expect("Missing count line").unwrap();
    let count = get_usize_numbers(&count_line)[0];

    // Read strings
    for i in 0..count {
        let possible_line = stdin_iter.next();
        let word = possible_line.expect("Missing word line").unwrap();
        strings.push(word);
        indexes.push(Vec::new());
    }
    
    // Join strings
    loop {
        let possible_line = stdin_iter.next();
        if possible_line.is_none() {
            break;
        }
        let command_line = possible_line.expect("Missing count line").unwrap();
        let numbers = get_usize_numbers(&command_line);
        let target = numbers[0] - 1;
        let source = numbers[1] - 1;
        if let Some(inner_vector) = indexes.get_mut(target) {
            inner_vector.push(source);
        }
        final_line = target;
    }


    let mut result = String::with_capacity(100000);
    print_recursive(final_line, &indexes, &strings, &mut result);
    println!("{}", result);

    ()
}

fn print_recursive(index: usize, indexes: &Vec<Vec<usize>>, strings: &Vec<String>, result: &mut String) {
    result.push_str(&strings[index]);
    for &u in indexes[index].iter() {
        print_recursive(u, indexes, strings, result);
    }
}

fn get_usize_numbers(s: &str) -> Vec<usize> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<usize>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

#[cfg(test)]
mod tests;
