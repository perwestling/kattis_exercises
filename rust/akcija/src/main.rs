use std::{io, io::BufRead};

#[allow(unused)]
pub fn main() {
    let mut line = String::new();
    let mut cursor = io::stdin().lock();

    let mut book_groups : Vec<Vec<u32>> = Vec::new();
    let mut prices : Vec<u32> = Vec::new();
    
    _ = cursor.read_line(&mut line).expect("Failed to read line");
    let book_count = line.trim().parse::<u32>().unwrap();

    let groups = book_count / 3;
    for i in 0..groups {
        book_groups.push(Vec::new());
    }

    // Read strings
    for _i in 0..book_count {
        line.clear();
        _ = cursor.read_line(&mut line).expect("Failed to read line");
        let price = line.trim().parse::<u32>().unwrap();
        prices.push(price);
    }
    prices.sort_by(|a,b| b.cmp(a));

    // We will now group the prices by three and add the first
    // two to the price; the 3rd is free
    let mut total_price : u32 = 0;
    for i in 0..book_count {
        if (i + 1) % 3 == 0 {
            continue;
        }
        total_price += prices[i as usize];
    }

    // Final price
    println!("{}", total_price);

    ()
}

#[cfg(test)]
mod tests;
