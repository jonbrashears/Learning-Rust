use std::thread;
use std::sync::mpsc;


fn main() {
    let nthreads = 5;   // Number of threads we will spawn
    let (tx, rx) = mpsc::channel();     // Define our channels

    for i in 0..nthreads {
        let tx = tx.clone();
        thread::spawn(move || {
            let response = format!("hello {}", i);
            tx.send(response).unwrap();                             // Send our response
        });
    }

    for _ in 0..nthreads {
        println!("got {:?}", rx.recv());                            // Receive sent response
    }
}