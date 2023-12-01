// The input file contains lines of digits and letters
// We need to extract the first and last digit to form a two-digit number
// The sum of all those numbers is our output.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    let mut sum = 0;

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut match_iterator = line.matches(char::is_numeric);
            let first_match = match_iterator.next();
            let last_match = match_iterator.next_back();

            let mut first_digit = 0;

            println!("Current line is {}", line);
            match first_match.expect("Each line should have at least one digit").parse::<u32>() {
                Ok(num) => {
                    sum += 10 * num;
                    first_digit = num;
                },
                Err(_) => println!("Could not parse {} as integer", first_match.unwrap()),
            };

            match last_match {
                Some(letter) => match letter.parse::<u32>() {
                    Ok(num) => sum += num,
                    Err(_) => println!("Could not parse {} as integer", last_match.unwrap()),
                },
                None => {
                    // This can happen if the line only has a single digit.
                    // In that case we add the first digit again.
                    sum += first_digit;
                }
            };

        }
    }

    println!("Total sum is {}", sum);


}
