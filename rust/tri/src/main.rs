use std::{io, io::BufRead};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let line = stdin_iter.next().expect("Missing line 1").unwrap();
    let numbers = get_i32_numbers(&line);
    let x = numbers[0];
    let y = numbers[1];
    let z = numbers[2];
    try_addition(x, y, z);
    ()
}

fn get_i32_numbers(s: &str) -> Vec<i32> {
    return s
        .to_string()
        .split_whitespace()
        .map(|item| item.to_string().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn try_addition(x: i32, y: i32, z: i32) -> () {
    if x + y == z {
        println!("{}+{}={}", x, y, z);
        return;
    }
    if x == y + z {
        println!("{}={}+{}", x, y, z);
        return;
    }
    try_subtraction(x, y, z);
}

fn try_subtraction(x: i32, y: i32, z: i32) -> () {
    if x - y == z {
        println!("{}-{}={}", x, y, z);
        return;
    }
    if x == y - z {
        println!("{}={}-{}", x, y, z);
        return;
    }
    try_division(x, y, z);
}

fn try_division(x: i32, y: i32, z: i32) -> () {
    if y != 0 && x / y == z {
        println!("{}/{}={}", x, y, z);
        return;
    }
    if z != 0 && x == y / z {
        println!("{}={}/{}", x, y, z);
        return;
    }
    try_multiplication(x, y, z);
}

fn try_multiplication(x: i32, y: i32, z: i32) -> () {
    if x * y == z {
        println!("{}*{}={}", x, y, z);
        return;
    }
    if x == y * z {
        println!("{}={}*{}", x, y, z);
        return;
    }
    println!("Error!");
}

#[cfg(test)]
mod tests;
