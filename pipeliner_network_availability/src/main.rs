extern crate pipeliner;
use pipeliner::Pipeline;

use std::net::*;

fn main() {
    // Vector of address to check
    let addresses: Vec<_> = (1..40).map(|n| format!("192.168.0.{}:0", n)).collect();
    let n = addresses.len();

    // Spawn a thread to check each socket
    for result in addresses.with_threads(n).map(|s| s.to_socket_addrs().unwrap().next().unwrap()) {
        println!("got: {:?}", result);
    }
}