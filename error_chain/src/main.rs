#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }

        errors {
            // Error defined for the user not providing a filename
            NoArgument(t: String) {
                display("no argument provided: '{}'", t)
            }
        }
    }
}
use errors::*;

fn run() -> Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    // Look in args for file name. If no name is found, throw NoArgument error
    let file = args().skip(1).next().ok_or(ErrorKind::NoArgument("filename needed".to_string()))?;

    // Open file
    let f = File::open(&file)?;

    // Read in lines of file, up to 10 lines
    let mut l = 0;
    for line in BufReader::new(f).lines() {
        let line = line?;
        println!("{}", line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("error {}",e);

        // Handles our defined errors
        match e.kind() {
            &ErrorKind::Msg(ref s) => println!("msg {}", s),
            &ErrorKind::Io(ref s) => println!("io {}", s),
            &ErrorKind::NoArgument(ref s) => println!("no argument {:?}", s),
            _ => ()
        }
        // Exits if we throw an error
        std::process::exit(1);
    }
}
