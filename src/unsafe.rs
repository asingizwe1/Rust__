fn main(){
let arr =[1,2,3];

    // Safe way: Rust checks bounds
    println!("Safe access: {}", arr[2]); // prints 30
  // Unsafe way: bypasses bounds checking
    let index = 5;

    unsafe{//block making you cater for safety rules
 let value = arr.get_unchecked(index);//get_unchecked -> ignores out of bounds check
        println!("Unsafe access: {}", value);
/*impl<T> [T] {
    pub unsafe fn get_unchecked(&self, index: usize) -> &T
} */
    }

}