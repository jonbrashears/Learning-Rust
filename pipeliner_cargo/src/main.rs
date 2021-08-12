extern crate pipeliner;
use pipeliner::Pipeline;

use std::process::Command;

fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    // Send command to shell.
    let output = Command::new("bin/sh")         
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout).trim_end().to_string(),
        output.status.success()
    )
}

fn main() {
    // Vector of commands
    let addresses: Vec<_> = (1..40).map(|n| format!("ping -c1 192.168.0.{}", n)).collect();
    let n = addresses.len();

    // Ping each address using the shell fn
    for result in addresses.with_threads(n).map(|s| shell(&s)) {
        if result.1 {
            println!("got: {}", result.0);
        }
    }
}