use std::{io, io::BufRead};
// use std::{io};

fn main() {
    let mut line = String::new();
    let mut cursor = io::stdin().lock();
    let mut queens : Vec<(usize, usize)> = Vec::new();
    
    // Read strings
    for row in 0..8 {
        line.clear();
        _ = cursor.read_line(&mut line).expect("Failed to read line");
        let chars: Vec<char> = line.trim().chars().collect::<Vec<_>>();
        let mut column : usize = 0;
        for &c in chars.iter() {
            column += 1;
            if c == '*' {
                queens.push((row, column));
            }
        }
    }
    
    if check_queens(&queens) {
        println!("valid");
    }
    else {
        println!("invalid");
    }
}

fn check_queens(queens: &Vec<(usize, usize)>) -> bool {
    if queens.len() != 8 {
        return false;
    }
    return check_valid(&queens[..]);
}

fn check_valid(queens: &[(usize, usize)]) -> bool {
    if queens.len() < 2 {
        return true;
    }
    let &(row, column) = &queens[0];
    let tail = &queens[1..];
    if check_collision_free((&row, &column), tail) {
        return check_valid(tail);
    }
    else {
        return false;
    }
}

fn check_collision_free((&row, &column): (&usize, &usize), remaining: &[(usize, usize)]) -> bool {
    for &(r, c) in remaining.iter() {
        if is_collision((&row, &column), (&r, &c)) {
            return false;
        }    
    }
    return true;
}

fn is_collision((&lr, &lc) : (&usize, &usize), (&rr, &rc) : (&usize, &usize)) -> bool {
    // Same row or column?
    if lr == rr || lc == rc {
        return true;
    }
    // Same diagonal
    let row_diff : i8 = (lr as i8) - (rr as i8);
    let column_diff : i8 = (lc as i8) - (rc as i8);
    return row_diff.abs() == column_diff.abs();
}

#[cfg(test)]
mod tests;
