use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act_on_input() {
        assert_eq!("To the convention.", act_on_input(17, 3));
        assert_eq!("Left beehind.", act_on_input(13, 14));
        assert_eq!("Never speak again.", act_on_input(8, 5));
        assert_eq!("Undecided.", act_on_input(44, 44));
    }
}

