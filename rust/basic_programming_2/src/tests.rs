use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_1_positive() {
        assert_eq!("Yes", act_on_input(1, 7, "1 7770 3 4 5 6 7"));
    }

    #[test]
    fn test_handle_1_negative() {
        assert_eq!("No", act_on_input(1, 7, "1 7770 3 4 5 6 6"));
    }

    #[test]
    fn test_handle_2_positive() {
        assert_eq!("Unique", act_on_input(2, 7, "1 7770 3 4 5 6 7"));
    }

    #[test]
    fn test_handle_2_negative() {
        assert_eq!("Contains duplicate", act_on_input(2, 7, "1 7770 3 4 5 6 6"));
    }

    #[test]
    fn test_handle_3_positive() {
        assert_eq!("1", act_on_input(3, 7, "1 1 2 2 1 2 1"));
    }

    #[test]
    fn test_handle_3_negative() {
        assert_eq!("-1", act_on_input(3, 7, "1 1 2 2 0 1 2"));
    }

    #[test]
    fn test_handle_4_odd() {
        assert_eq!("5", act_on_input(4, 7, "8 1 4 3 6 7 5"));
    }

    #[test]
    fn test_handle_4_even() {
        assert_eq!("4 5", act_on_input(4, 8, "8 1 4 3 6 7 5 2"));
    }

    #[test]
    fn test_handle_5() {
        assert_eq!("210 321 543 777 999", act_on_input(5, 7, "210 999 1000 543 321 99 777"));
    }
}

