use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io;

// Reads a line from the Tcp Stream
fn handle_connection(stream: TcpStream) -> io::Result<()>{
    // Reading the stream consumes it, so we have to clone the stream we wish to reply to
    let mut ostream = stream.try_clone()?;
    let mut rdr = io::BufReader::new(stream);
    let mut text = String::new();
    rdr.read_line(&mut text)?;
    println!("got '{}", text.trim_end());

    // Echo received line back to client
    ostream.write_all(text.as_bytes())?;
    Ok(())
}

fn main() {
    // Start server and listen on port
    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    println!("error {:?}", e);
                }
            }
            Err(e) => { println!("connection failed {}", e);}
        }
    }
}