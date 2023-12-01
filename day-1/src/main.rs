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
            // List of tuples of match pattern and their numeric value
            let patterns = [
                ("one", 1), ("1", 1),
                ("two", 2), ("2", 2),
                ("three", 3), ("3", 3),
                ("four", 4), ("4", 4),
                ("five", 5), ("5", 5),
                ("six", 6), ("6", 6),
                ("seven", 7), ("7", 7),
                ("eight", 8), ("8", 8),
                ("nine", 9), ("9", 9),
            ];

            let mut index_first_match = usize::MAX;
            let mut value_first_match = 0;
            let mut index_last_match = usize::MIN;
            let mut value_last_match = 0;
            for (pattern, value) in patterns {
                match line.match_indices(pattern).next() {
                    Some((i, _)) => {
                        if i <= index_first_match {
                            index_first_match = i;
                            value_first_match = value;
                        }
                    },
                    None => ()
                };
                match line.rmatch_indices(pattern).next() {
                    Some((i, _)) => {
                        if i >= index_last_match {
                            index_last_match = i;
                            value_last_match = value;
                        }
                    },
                    None => ()
                };
            }

            println!("Found in {}: {}{}", line, value_first_match, value_last_match);

            sum += 10 * value_first_match + value_last_match;
        }
    }

    println!("Total sum is {}", sum);


}
