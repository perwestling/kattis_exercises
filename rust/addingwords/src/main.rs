use std::{io, io::BufRead, collections::HashMap};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let mut word_to_number : HashMap<String, i32> = HashMap::new();
    let mut number_to_word : HashMap<i32, String> = HashMap::new();

    loop {
        let possible_line = stdin_iter.next();
        if possible_line.is_none() {
            return ();
        }
        let word_line = possible_line.expect("Missing word line").unwrap();
        let mut words = word_line.split(" ").collect::<Vec<&str>>();
        let command = words.remove(0);
        match command {
            "def" => {
                handle_def(&words, &mut word_to_number, &mut number_to_word);
            },
            "calc" => {
                handle_calc(&mut words, &word_to_number, &number_to_word);
                println!("{}", words.join(" "));
            },
            "clear" => {
                handle_clear(&mut word_to_number, &mut number_to_word);
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

fn handle_def<'a>(words: &Vec<&'a str>,
                  word_to_number: &'a mut HashMap<String, i32>,
                  number_to_word: &'a mut HashMap<i32, String>) -> () {
    let word = words[0];
    match word_to_number.get(word) {
        None => {
            ();
        },
        Some(number) => {
            number_to_word.remove(number);
        }
    }
    let number = words[1].parse::<i32>().unwrap();
    word_to_number.insert(word.to_string(), number);
    number_to_word.insert(number, word.to_string());
}

fn handle_calc<'a>(words: &mut Vec<&'a str>,
                   word_to_number: &'a HashMap<String, i32>,
                   number_to_word: &'a HashMap<i32, String>) {
    do_handle_calc(0, 0, None, words, word_to_number, number_to_word);
}

fn do_handle_calc<'a>(mut result : i32,
                     index : usize,
                     is_plus : Option<bool>,
                     words: &mut Vec<&'a str>,
                     word_to_number: &'a HashMap<String, i32>,
                     number_to_word: &'a HashMap<i32, String>) {
    if words[index] == "=" {
        match number_to_word.get(&result) {
            Some(word) => {
                words.push(word);
            },
            None => {
                words.push("unknown");
            }
        }
    }
    else if index % 2 == 0 {
        // Word
        let word = words[index];
        match (word_to_number.get(word), is_plus) {
            (Some(&number), Some(false)) => {
                result = result - number;
            },
            (Some(&number), _) => {
                result = result + number;
            },
            (None, _) => {
                words.push("unknown");
                return;
            }
        }
        do_handle_calc(result, index + 1, None, words, word_to_number, number_to_word);
    }
    else {
        // + or -
        let is_plus = words[index] == "+";
        do_handle_calc(result, index + 1, Some(is_plus), words, word_to_number, number_to_word);
    }
}

fn handle_clear<'a>(word_to_number: &'a mut HashMap<String, i32>, number_to_word: &'a mut HashMap<i32, String>) {
    word_to_number.clear();
    number_to_word.clear();
}

#[cfg(test)]
mod tests;
