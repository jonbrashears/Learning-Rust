use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

fn main() {
    // Check for filename arguement. If none is given, use the default
    let filename = std::env::args().nth(1).unwrap_or("sherlock.txt".to_string());
    let file_to_read: String = filename.parse().expect("File name must be a string!");
    let mut f = File::open(file_to_read).expect("That file could not be open");

    // Read in the file to a string
    let mut text = String::new();
    f.read_to_string(&mut text).expect("Could not read file!");

    let mut map = HashMap::new();
    
    // Split whenever a character is NOT alphabetical
    for s in text.split(|c: char| ! c.is_alphabetic()) {               
        let word = s.to_lowercase();                                 // Convert to lowercase to compare
        let count = map.entry(word).or_insert(0);       // Count number of times the word appears. If word is not found, insert it
        *count += 1;
    }

    println!("Total words: {}", map.len());

    let mut entries: Vec<_> = map.into_iter().collect();                       // Convert into vector of key, value tuples
    entries.sort_by(|a, b| b.1.cmp(&a.1));   // Sort in descending order

    // Print the twenty most frequent words and their count
    for e in entries.iter().take(20) {                            
        println!("{}\t{}", e.0, e.1);
    }

}
