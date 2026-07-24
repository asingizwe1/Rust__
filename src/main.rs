//threads in rust
//every program starts with one thread known as the main thread
use std::thread;

//spawn takes the function the new thread will execture as the parameter
fn main() {
    thread::spawn(f);
    thread::spawn(f);
    println!("main thread");
}

fn f() {
    println!("another thread");
    let id = thread::current().id();
    println!("thread id: {id:?}");
}
