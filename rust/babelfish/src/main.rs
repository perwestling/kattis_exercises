use std::{io, io::BufRead, collections::BTreeMap};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let mut dictionary = BTreeMap::new();
    let mut answers : Vec<&str> = Vec::new();

    // Read dictionary
    loop {
        let possible_line = stdin_iter.next();
        let word_line = possible_line.expect("Missing word line").unwrap();
        let words = word_line.split(" ").collect::<Vec<&str>>();
        if words.len() < 2 {
            break;
        }
        let english_word = words[0].into();
        let foreign_word = words[1].into();
        dictionary.insert(foreign_word, english_word);
    }

    // Translate words
    loop {
        let possible_line = stdin_iter.next();
        if possible_line.is_none() {
            break;
        }
        let word = possible_line.expect("Missing word line").unwrap();
        answers.push(act_on_input(&word, &dictionary));
    }

    let _ = io::stdout().lock();
    for answer in answers.iter() {
        println!("{}", answer);
    }

    ()
}
fn act_on_input<'a>(input: &str, dictionary: &'a BTreeMap<String, String>) -> &'a str {
    return match dictionary.get(input) {
        Some(translation) => translation,
        None => "eh"
    }
}

#[cfg(test)]
mod tests;
