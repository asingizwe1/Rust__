//Refcell(for complex datatypes) ->lets you borrow mutable or immutable references even through a shared reference, but it checks the rules at runtime instead of compile time.
//.borrow() → increases the “immutable borrow” count. .borrow_mut() → requires that no immutable borrows are active, and locks it for one mutable borrow.
use std::cell::RefCell;
fn main() {
    // With Cell<Vec<i32>>
    let c = Cell::new(Vec::new());
    let mut temp = c.take(); // must take whole Vec out
    temp.push(1);
    c.set(temp); // put it back

    // With RefCell<Vec<i32>>
    let r = RefCell::new(Vec::new());
    r.borrow_mut().push(1); // directly mutate inside
    println!("Values: {:?}", r.borrow()); // [1]
}
//&T (shared reference) → you can only read.
//&mut T (exclusive reference) → you can mutate, but only one at a time.
//refcell
/*
Cell<T> when:

You only need to move whole values in/out.

T is Copy or you’re okay with replacing/taking the entire value.

RefCell<T> when:

You need to borrow references to the inside (not just swap whole values).

You want to mutate complex structures (like a Vec, HashMap, or custom struct).
*/
//RefCell lets you borrow references to the inside-> you can peek inside and work directly with the inner value (T) using normal references (&T or &mut T).

/*.borrow() → gives you an immutable reference (&T) to the inside.
Example: read the contents safely.

.borrow_mut() → gives you a mutable reference (&mut T) to the inside.
Example: push to a Vec, change a field, etc.

.into_inner() → consumes the RefCell and gives you the owned value back.
Example: take the whole thing out when you’re done. */

//CELL
/*With Cell<T>, you can’t peek inside directly.
You have to take the whole value out, work on it, then put it back.
Example:

rust
let c = Cell::new(Vec::new());
let mut temp = c.take(); // pull the Vec out
temp.push(1);
c.set(temp);             // put it back */

//REFCELL
/*RefCell<T>, you don’t need to pull the whole thing out.
You can borrow a reference to the inside and work with it directly.
Example:

rust
let r = RefCell::new(Vec::new());
r.borrow_mut().push(1);  // directly mutate inside
println!("{:?}", r.borrow()); // read inside


Cell → you swap the whole Vec struct in/out.

RefCell → you borrow a reference to the Vec struct and work directly with its pointer-managed heap data.

*/
