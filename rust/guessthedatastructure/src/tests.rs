use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_case_pairs(case: u8) -> Vec<(i32, i32)> {
        println!("Try case {}", case as u32);
        let mut result : Vec<(i32, i32)> = Vec::new();
        match case {
            1 => {
                result.push((1, 1));
                result.push((1, 2));
                result.push((1, 3));
                result.push((2, 1));
                result.push((2, 2));
                result.push((2, 3));
            },
            2 => {
                result.push((1, 1));
                result.push((1, 2));
                result.push((1, 3));
                result.push((2, 3));
                result.push((2, 2));
                result.push((2, 1));
            },
            3 => {
                result.push((1, 1));
                result.push((2, 2));
            },
            4 => {
                result.push((1, 2));
                result.push((1, 1));
                result.push((2, 1));
                result.push((2, 2));
            },
            5 => {
                result.push((1, 2));
                result.push((1, 5));
                result.push((1, 1));
                result.push((1, 3));
                result.push((2, 5));
                result.push((1, 4));
                result.push((2, 4));
            },
            6 => {
                result.push((2, 1));
            },
            _ => panic!("Unsupported!"),
        }
        return result;
    }

    #[test]
    fn test_get_verdict_stack() {
        assert_eq!(false, get_verdict_stack(&get_case_pairs(1)));
        assert_eq!(true, get_verdict_stack(&get_case_pairs(2)));
        assert_eq!(false, get_verdict_stack(&get_case_pairs(3)));
        assert_eq!(true, get_verdict_stack(&get_case_pairs(4)));
        assert_eq!(false, get_verdict_stack(&get_case_pairs(5)));
        assert_eq!(false, get_verdict_stack(&get_case_pairs(6)));
    }
    
    #[test]
    fn test_get_verdict_queue() {
        assert_eq!(true, get_verdict_queue(&get_case_pairs(1)));
        assert_eq!(false, get_verdict_queue(&get_case_pairs(2)));
        assert_eq!(false, get_verdict_queue(&get_case_pairs(3)));
        assert_eq!(false, get_verdict_queue(&get_case_pairs(4)));
        assert_eq!(false, get_verdict_queue(&get_case_pairs(5)));
        assert_eq!(false, get_verdict_queue(&get_case_pairs(6)));
    }

    #[test]
    fn test_get_verdict_prio_queue() {
        assert_eq!(false, get_verdict_prio_queue(&get_case_pairs(1)));
        assert_eq!(true, get_verdict_prio_queue(&get_case_pairs(2)));
        assert_eq!(false, get_verdict_prio_queue(&get_case_pairs(3)));
        assert_eq!(false, get_verdict_prio_queue(&get_case_pairs(4)));
        assert_eq!(true, get_verdict_prio_queue(&get_case_pairs(5)));
        assert_eq!(false, get_verdict_prio_queue(&get_case_pairs(6)));
    }

    #[test]
    fn test_act_on_input() {
        assert_eq!("queue", act_on_input(&get_case_pairs(1)));
        assert_eq!("not sure", act_on_input(&get_case_pairs(2)));
        assert_eq!("impossible", act_on_input(&get_case_pairs(3)));
        assert_eq!("stack", act_on_input(&get_case_pairs(4)));
        assert_eq!("priority queue", act_on_input(&get_case_pairs(5)));
        assert_eq!("impossible", act_on_input(&get_case_pairs(6)));
    }
}

