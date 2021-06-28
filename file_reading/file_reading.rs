use std::fs::File;
use std::io;
use std::io::prelude::*;

// Generic struct for obects that implement Read
struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String
}

// Lines structure implementations
impl <R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines{reader: io::BufReader::new(r), buf: String::new()}
    }

    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>>{
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => if nbytes == 0 {          // If we get Ok and no bytes, we have reached the end of file
                None                                // EOF, return None
            } else {                                // If we get Ok and some bytes back
                let line = self.buf.trim_end();     // Get the line and trim off the line feed
                Some(Ok(&line))                     // Return Some
            },
            Err(e) => Some(Err(e))                  // If we error
        }
    }
}

// Reads each line from a file and prints it
fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;              // Open file
    let mut lines = Lines::new(file);               
    while let Some(line) = lines.next() {           // Loop until we reach EOF
        let line = line?;                           // Unwrap our resulting line
        println!("{}", line);
    }
    Ok(())
}

fn main() {
    let filename = std::env::args().nth(1).expect("Please supply a path and name");
    let file_to_read: String = filename.parse().expect("Filename must be in the form of a string");
    let read_file = read_all_lines(&file_to_read);

    if read_file.is_ok() {
        println!("Successflly parsed file");
    } else {
        println!("Failed to parse file: {:?}", read_file);
    }
}
