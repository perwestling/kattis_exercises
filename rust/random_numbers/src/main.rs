use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::io::stdin;

fn main() -> io::Result<()> {
    // Ask the user for the number of random numbers to generate
    let mut input = String::new();
    println!("Enter the number of random numbers to generate:");
    stdin().read_line(&mut input).expect("Failed to read line");
    let count: usize = input.trim().parse().expect("Please enter a valid number");

    // Create a file to write the numbers
    let filename = format!("numbers{}.txt", count);
    let mut file = File::create(&filename)?;

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // First line should be the number of lines in the file
    writeln!(file, "{}", count)?;

    // Generate and write the specified number of random numbers between 1 and 100,000
    for _ in 0..count {
        let number: u32 = rng.gen_range(1..=100_000);
        writeln!(file, "{}", number)?;
    }

    println!("{} random numbers have been written to {}", count, filename);
    Ok(())
}
