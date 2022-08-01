use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut b = blockchain::Blockchain::new_blockchain();
    thread::sleep(Duration::from_secs(5));
    b.add("a -> b 3 btc".to_string());
    thread::sleep(Duration::from_secs(5));
    b.add("c -> b 5btc".to_string());
}
