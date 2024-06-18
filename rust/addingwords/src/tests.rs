use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_def() {
        let mut word_to_number = HashMap::new();
        let mut number_to_word = HashMap::new();
        handle_def(&vec!["foo", "3"], &mut word_to_number, &mut number_to_word);
        handle_def(&vec!["bar", "7"], &mut word_to_number, &mut number_to_word);
        handle_def(&vec!["programming", "10"], &mut word_to_number, &mut number_to_word);
        assert_eq!(Some(&3), word_to_number.get("foo"));
        assert_eq!(Some(&7), word_to_number.get("bar"));
        assert_eq!(Some(&10), word_to_number.get("programming"));
        assert_eq!("foo", number_to_word.get(&3).unwrap().as_str());
        assert_eq!("bar", number_to_word.get(&7).unwrap().as_str());
        assert_eq!("programming", number_to_word.get(&10).unwrap().as_str());
    }

    #[test]
    fn test_handle_def_redone() {
        let mut word_to_number = HashMap::new();
        let mut number_to_word = HashMap::new();
        handle_def(&vec!["foo", "3"], &mut word_to_number, &mut number_to_word);
        handle_def(&vec!["foo", "7"], &mut word_to_number, &mut number_to_word);
        handle_def(&vec!["foo", "10"], &mut word_to_number, &mut number_to_word);
        assert_eq!(Some(&10), word_to_number.get("foo"));
        assert_eq!(None, number_to_word.get(&3));
        assert_eq!(None, number_to_word.get(&7));
        assert_eq!("foo", number_to_word.get(&10).unwrap().as_str());
    }

    #[test]
    fn test_handle_calc() {
        let mut word_to_number = HashMap::new();
        let mut number_to_word = HashMap::new();
        word_to_number.insert("foo".to_string(), 3);
        word_to_number.insert("bar".to_string(), 7);
        word_to_number.insert("programming".to_string(), 10);
        number_to_word.insert(3, "foo".to_string());
        number_to_word.insert(7, "bar".to_string());
        number_to_word.insert(10, "programming".to_string());
        let mut words : Vec<&str> = vec!["foo", "+", "bar", "="];
        handle_calc(&mut words, &word_to_number, &number_to_word);
        assert_eq!(words, ["foo", "+", "bar", "=", "programming"]);
    }

    #[test]
    fn test_handle_clear() {
        let mut word_to_number = HashMap::new();
        let mut number_to_word = HashMap::new();
        word_to_number.insert("foo".to_string(), 3);
        word_to_number.insert("bar".to_string(), 7);
        word_to_number.insert("programming".to_string(), 10);
        number_to_word.insert(3, "foo".to_string());
        number_to_word.insert(7, "bar".to_string());
        number_to_word.insert(10, "programming".to_string());
        handle_clear(&mut word_to_number, &mut number_to_word);
        assert!(word_to_number.is_empty());
        assert!(number_to_word.is_empty());
    }
}

