use std::sync::Mutex; // synchronization primitive used in Rust (and many other languages) to protect shared data from being accessed by multiple threads at the same time.
use std::thread;
//Ten threads incrementing a counter safely:
fn main() {
    let balance = Mutex::new(1000);
    //s constructs a new Mutex<i32> that wraps the integer 1000.

    // temporary region (scope) where you can spawn threads.
    //All threads inside must finish before the scope ends.
    thread::scope(|s| {
        // Two threads trying to withdraw
        s.spawn(|| {
            let mut b = balance.lock().unwrap();
            *b -= 200; // withdraw
        });
        s.spawn(|| {
            let mut b = balance.lock().unwrap();
            *b -= 300; // withdraw
        }); //both threads would read same balance without mutex
    });
    println!("Final balance: {}", *balance.lock().unwrap());
}
