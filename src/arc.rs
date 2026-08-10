//Rc usecase
//Reference counted
//Both Rc and Arc are reference‑counted smart pointers.
use std::rc::Rc;///Rc is for single threaded programmes
fn main(){
let b =Rc::new("hello");
let a = b.clone();//counter=2
//clone give you another pointer BUT DOESNT DUPLICATE THE DATA
println!("Count: {}", Rc::strong_count(&a)); // 2

    drop(b); // counter = 1
    println!("Count: {}", Rc::strong_count(&a)); // 1


}