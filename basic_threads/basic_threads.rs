use std::thread;
use std::sync::Arc;

struct MyString(String);

impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}

fn main() {

    // Vector for threads
    let mut threads = Vec::new();

    // Atomic reference counting
    let name = Arc::new(MyString::new("dolly"));

    // Spawn 5 threads (0-4)
    for i in 0..5 {
        let tname = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}", tname.0, i);
        });

        threads.push(t);
    }

    // Handler for join failing
    for t in threads {
        t.join().expect("thread failed");
    }
}