use super::*;

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

