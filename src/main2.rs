//If we want to make sure the threads are finished before we return from main, we can wait for them by joining them.
//use the JoinHandle returned by the spawn function
use std::thread;

//to wait for the thread just join
//.join() method waits until the thread has finished executing and returns a std::thread::Result.
fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello from the main thread.");
    t1.join().unwrap(); //join takes ownership of thread
    t2.join().unwrap(); //you've consumed t2
} //join to ensure that the thread completes
fn f() {
    println!("another thread");
    let id = thread::current().id();
    println!("thread id: {id:?}");
}
