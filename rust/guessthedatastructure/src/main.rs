use std::{io, io::BufRead, collections::VecDeque, collections::BinaryHeap};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    loop {
        let possible_line = stdin_iter.next();
        if possible_line.is_none() {
            return ();
        }
        let number_line = possible_line.expect("Missing count line").unwrap();
        let count_line = get_i32_numbers(&number_line);
        let n : i32 = count_line[0];
        let mut pairs : Vec<(i32, i32)> = Vec::new();
        for _x in 0..n {
            let pair_line = stdin_iter.next().expect("Missing pair line").unwrap();
            let pair_numbers = get_i32_numbers(&pair_line);
            let (command, item) = (pair_numbers[0], pair_numbers[1]);
            pairs.push((command, item));
        }
        println!("{}", act_on_input(&pairs));
    }
}

fn get_i32_numbers(s: &str) -> Vec<i32> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<i32>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn act_on_input(pairs: &Vec<(i32, i32)>) -> &str {
    let verdict_stack : bool = get_verdict_stack(&pairs);
    let verdict_queue : bool = get_verdict_queue(&pairs);
    let verdict_priority_queue : bool = get_verdict_prio_queue(&pairs);
    return match (verdict_stack, verdict_queue, verdict_priority_queue) {
        (false, false, false) =>
            "impossible",
        (false, false, true) =>
            "priority queue",
        (false, true, false) =>
            "queue",
        (true, false, false) =>
            "stack",
        (false, true, true) | (true, false, true) | (true, true, false) | (true, true, true) =>
            "not sure",
    }
}

fn get_verdict_stack(pairs: &Vec<(i32, i32)>) -> bool {
    let mut stack : VecDeque<&i32> = VecDeque::new();
    for pair in pairs {
        match pair {
            (1, number) => { stack.push_front(number); },
            (2, number) => {
                    if stack.pop_front() != Some(number) {
                        return false;
                    }
                },
            _ => {
                panic!("Invalid command for stack");
            },
        }
    }
    return true;
}

fn get_verdict_queue(pairs: &Vec<(i32, i32)>) -> bool {
    let mut queue : VecDeque<&i32> = VecDeque::new();
    for pair in pairs {
        match pair {
            (1, number) => { queue.push_front(number); },
            (2, number) => {
                    if queue.pop_back() != Some(number) {
                        return false;
                    }
                },
            _ => {
                panic!("Invalid command for queue");
            },
        }
    }
    return true;
}

fn get_verdict_prio_queue(pairs: &Vec<(i32, i32)>) -> bool {
    let mut prio_queue : BinaryHeap<&i32> = BinaryHeap::new();
    for pair in pairs {
        match pair {
            (1, number) => { prio_queue.push(number); },
            (2, number) => {
                    if prio_queue.pop() != Some(number) {
                        return false;
                    }
                },
            _ => {
                panic!("Invalid command for prio queue");
            },
        }
    }
    return true;
}

#[cfg(test)]
mod tests;
