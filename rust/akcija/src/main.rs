use std::{io, io::BufRead, time::Instant};
// use std::{io, io::BufRead};

pub fn main() {
    // let start = Instant::now();
    let mut line = String::new();
    let mut cursor = io::stdin().lock();

    let mut book_groups : Vec<Vec<u32>> = Vec::new();
    let mut prices : Vec<u32> = Vec::new();
    
    _ = cursor.read_line(&mut line).expect("Failed to read line");
    let book_count = line.trim().parse::<u32>().unwrap();

    let groups = book_count / 3;
    for _ in 0..groups {
        book_groups.push(Vec::new());
    }

    // Read strings
    for _ in 0..book_count {
        line.clear();
        _ = cursor.read_line(&mut line).expect("Failed to read line");
        let price = line.trim().parse::<u32>().unwrap();
        prices.push(price);
    }
    // prices.sort_by(|a,b| b.cmp(a));
    radix_sort_descending(&mut prices);

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

    // let duration = start.elapsed();
    // println!("Time elapsed is: {:?}", duration);
    
    ()
}

fn radix_sort_descending(arr: &mut [u32]) {
    let max_num = match arr.iter().max() {
        Some(&max) => max,
        None => return,
    };

    let mut exp = 1;
    while max_num / exp > 0 {
        counting_sort_descending(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_descending(arr: &mut [u32], exp: u32) {
    let mut output = vec![0; arr.len()];
    let mut count = [0; 10];  // Use fixed-size array for counts

    // Store count of occurrences in count[]
    for &number in arr.iter() {
        let index = (number / exp % 10) as usize;
        count[index] += 1;
    }

    // Change count[i] so that count[i] contains the actual
    // position of this digit in output[]
    for i in (0..9).rev() {
        count[i] = count[i + 1] + count[i];
    }

    // Build the output array in descending order
    for &number in arr.iter().rev() {
        let index = (number / exp % 10) as usize;
        output[count[index] - 1] = number;
        count[index] -= 1;
    }

    // Copy the output array to arr[], so that arr[] now
    // contains sorted numbers according to the current digit
    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod tests;
