//PROBLEM-> value has one owner
//When that owner goes out of scope, the value is dropped (memory freed).

//SOLUTION ->Rc<T> (Reference Counted):
//Think of it like a smart pointer that keeps a counter of how many owners exist.
//clone an Rc, it doesn’t copy the data — it just increments the counter.
//When an Rc is dropped, the counter decrements.
// use std::rc::Rc;

/* counter in Rc is just a normal integer.
If two threads tried to increment/decrement it at the same time:

One update could overwrite the other.

You’d get a race condition → memory freed too early or leaked. */

// use std::rc::Rc;

// fn main() {
//     let a = Rc::new("hello");
//     let b = a.clone(); // counter = 2
//     println!("Count: {}", Rc::strong_count(&a)); // 2

//     drop(b); // counter = 1
//     println!("Count: {}", Rc::strong_count(&a)); // 1
// } //DEALLOCATING will be when last rc is dropped
use std::sync::Arc; //counter is updated using atomic operations
use std::thread;

fn main() {
    let a = Arc::new([1, 2, 3]); // counter = 1
    let b = a.clone(); // counter = 2

    let t1 = thread::spawn(move || dbg!(a));
    let t2 = thread::spawn(move || dbg!(b));

    t1.join().unwrap();
    t2.join().unwrap();
} // both dropped → counter = 0 → memory freed
