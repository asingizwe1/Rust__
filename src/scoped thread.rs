fn main() {
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        //1
        //s represents the scope, we use s to spawn threads
        s.spawn(|| {
            //2
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    }); //3
}
// 1) We call the std::thread::scope function with a closure. Our closure is directly
// executed and gets an argument, s, representing the scope.
// 2) We use s to spawn threads. The closures can borrow local variables like numbers.
// 3) When the scope ends, all threads that haven’t been joined yet are automatically
// joined.

//rust gives you a scope handle s that you use to spawn threads
