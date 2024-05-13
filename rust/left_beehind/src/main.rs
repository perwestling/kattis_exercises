use std::{io, io::BufRead};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let mut answers : Vec<&str> = Vec::new();
    loop {
        let possible_line = stdin_iter.next();
        if possible_line.is_none() {
            return ();
        }
        let number_line = possible_line.expect("Missing count line").unwrap();
        let numbers = get_u32_numbers(&number_line);
        let sweet_honey = numbers[0];
        let sour_honey = numbers[1];
        if sweet_honey == 0 && sour_honey == 0 {
            let _ = io::stdout().lock();
            for answer in answers.iter() {
                println!("{}", answer);
            }
            return;
        }
        answers.push(&act_on_input(sweet_honey, sour_honey));
    }
}

fn get_u32_numbers(s: &str) -> Vec<u32> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<u32>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn act_on_input(sweet_honey: u32, sour_honey: u32) -> &'static str {
    if sweet_honey + sour_honey == 13 {
        return "Never speak again.";
    }
    else if sour_honey > sweet_honey {
        return "Left beehind.";
    }
    else if sweet_honey > sour_honey {
        return "To the convention.";
    }
    else {
        return "Undecided.";
    }
}

#[cfg(test)]
mod tests;
