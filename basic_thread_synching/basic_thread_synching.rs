use std::thread;
use std::sync::Arc;
use std::sync::Barrier;


fn main() {
    let nthreads = 5;   // Number of threads
    let mut threads = Vec::new();
    let barrier = Arc::new(Barrier::new(nthreads));

    for i in 0..nthreads {
        let barrier = barrier.clone();      // Create a barrier that will synchronize threads
        let t = thread::spawn(move || {
            println!("before wait {}", i);
            barrier.wait();                             // Wait for all threads to reach this point
            println!("after wait {}", i);               // Continue once all threads reach the barrier
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}