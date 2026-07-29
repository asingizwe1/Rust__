//when data is too big or with dynamic datatypes use Box
/*Data is too big for the stack.

You need recursive types.

You want trait objects.

You want explicit control of ownership. */

//Box::leak
//You can share that reference across threads safely, because it’s guaranteed to never disappear.

//Box::leak, you intentionally prevent Rust from ever freeing that memory
//. The value is turned into a reference with a 'static lifetime
//'static doesn’t mean “existed since program start.” It means: this value will live until program end.
fn main() {
    let msg: &'static str = Box::leak(Box::new(Sring::from("string")));
    // Spawn two threads that both use the leaked reference
    let t1 = thread::spawn(move || println!("Thread 1: {}", msg));
    let t2 = thread::spawn(move || println!("Thread 2: {}", msg));
    //pointer of smart pointer box, lives on the stack but actual value lives on heap
    //when box goes out of scope rust automatically drops it
    {
        let b = Box::new(42); // heap allocation
                              // b points to 42 on the heap
    } // b goes out of scope → memory freed
    t1.join().unwrap();
    t2.join().unwrap();
} /*let x: &'static i32 = Box::leak(Box::new(42));
  // x is a reference that will live until program ends */
