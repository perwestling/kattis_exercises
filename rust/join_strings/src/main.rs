use std::{io, io::BufRead};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let mut strings : Vec<String> = Vec::new();
    let mut indexes : Multimap<usize, usize> = Vec::new();
    let mut final_line : usize = 0;

    let possible_line = stdin_iter.next();
    let count_line = possible_line.expect("Missing count line").unwrap();
    let count = get_usize_numbers(&count_line)[0];
    
    // Read strings
    for i in 0..count {
        let possible_line = stdin_iter.next();
        let word = possible_line.expect("Missing word line").unwrap();
        strings.push(word);
        indexes.insert(i, i);
    }
    
    // Join strings
    loop {
        let possible_line = stdin_iter.next();
        if possible_line.is_none() {
            break;
        }
        let command_line = possible_line.expect("Missing count line").unwrap();
        let numbers = get_usize_numbers(&command_line);
        let target = numbers[0];
        let source = numbers[1];
        indexes.insert(target - 1, source - 1);
        // act_on_input(target, numbers[1], &mut dictionary);
        final_line = target;
    }

    let _ = io::stdout().lock();
    print_recursive(final_line, &dictionary);
    println!();

    ()
}

fn print_recursive(index: usize, dictionary: &Multimap<usize, usize>) {
    print!("{}", index);
    for i in dictionary.get_vec(index) {
        print_recursive(i, dictionary);
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
