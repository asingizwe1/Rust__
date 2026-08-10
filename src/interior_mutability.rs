//Interior mutability is a special trick where certain types let you mutate data even through a shared reference (&T).
/*Rust usually enforces “no mutation through &T” at compile time.

Interior mutability types (like Cell, RefCell, Mutex) bend this rule by saying
“Okay, you can mutate through a shared reference, but I’ll check the rules at runtime instead.” */

/*it only allows very controlled operations:

Copy types (Copy) → you can .get() the value out directly.

Non‑Copy types → you must replace the whole value (.set() or .take()), not borrow parts of it. 

If the item inside is small and copyable (like a number), you can peek (.get()) or swap (.set()).

If the item is bigger (like a vector), you must take the whole box out, change it, then put it back.


*/

use std::cell::Cell;//Cell<T> bends that rule: it lets you mutate through a shared reference
fn main(){
//COPY TYPE
    let a = Cell::new(10);

    // Shared reference, but we can still mutate
    a.set(20);
    println!("Value: {}", a.get()); // prints 20
//NON COPY TYPE
let v = Cell::new(Vec::new());
    // Take the Vec out (Cell leaves an empty Vec behind)
    let mut temp = v.take();//Moves the value out of the Cell, leaving a default empty value behind.
    //T must implement default so that Cell knows what to leave behind
    temp.push(1);

    // Put it back
    v.set(temp);

    println!("Value: {:?}", v.take()); // prints [1]


}