use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
fn main() {
    let large_vec = vec![0; 1024 * 1024 * 100];
    sleep(Duration::from_secs(30));
}
