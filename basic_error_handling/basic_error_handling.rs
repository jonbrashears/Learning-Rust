use std::error::Error;
use std::fmt;


// Struct for our new error definition
#[derive(Debug)]
struct MyError {
    details: String
}

// Constructor
impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

// Implement Display for our error struct
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

// Implement Error for our new error struct
impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

// Raises error "Broken" if MyError in our Result is true
// Else returns Ok
fn raises_my_error(yes: bool) -> Result<(), MyError> {
    if yes {
        Err(MyError::new("Broken"))
    } else {
        Ok(())
    }
}

use std::num::ParseFloatError;

// If ParseFloatError occurs, build a MyError and return it
impl From<ParseFloatError> for MyError {
    fn from(err: ParseFloatError) -> Self {
        MyError::new(&err.to_string())
    }
}

// Test function
// Notes: '?' Operator: If Ok -> unwrap and give innter value
//                      If Err -> Return Err from function
fn parse_f64(s: &str, yes: bool) -> Result<f64, MyError> {
    raises_my_error(yes)?;          // If we pass true, this raises an error
    let x: f64 = s.parse()?;        // If we cannot parse s as a float, this raises an error
    Ok(x)                           
}

fn main() {
    println!(" {:?}", parse_f64("42", false));
    println!(" {:?}", parse_f64("42", true));
    println!(" {:?}", parse_f64("?42", false));
}