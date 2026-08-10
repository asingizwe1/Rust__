//shared ownership in multi-threaded programs
//to bend ownership rules
// counter is updated using atomic operations.

use std::sync::Arc;
use std::thread;

fn main(){
let counter=Arc::new(42);
//each thread getting a reference by cloning

let c1=Arc::clone(&counter);
let c2=Arc::clone(&counter);

//spawn 2 threads that read the same data
let t1=thread::spawn(move||println!("{}",c1))
let t2=thread::spawn(move||println!("{}",c2))

//waiting for both threads to finish
 t1.join().unwrap();
    t2.join().unwrap();
//we still have access in the main
println!("Main thread sees counter = {}", counter);


}



























