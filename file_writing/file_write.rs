use std::fs::File;
use std::io;
use std::io::prelude::*;

fn write_out(f: &str) -> io::Result<()> {
    let mut out = File::create(f)?;             // Create the file to write to
    write!(out, "The answer is {}\n", 42)?;     // Write to file
    Ok(())                                      // Return Ok
}

fn main() {
    let filename = std::env::args().nth(1).expect("Please supply a path and name for the file you would like to create");
    let file_to_write: String = filename.parse().expect("Filename must be in the form of a string");
    write_out(&file_to_write).expect("Unable to write to file"); 
}