use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act_on_input() {
        let mut dictionary = BTreeMap::new();
        dictionary.insert("ogday".into(), "dog".into());
        dictionary.insert("atcay".into(), "cat".into());
        assert_eq!("cat", act_on_input("atcay", &dictionary));
        assert_eq!("eh", act_on_input("bt", &dictionary));
    }
}

