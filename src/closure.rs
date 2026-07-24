fn main() {
    let numbers = vec![1, 2, 3];

    std::thread::spawn(move || {
        // ✅ Closure can "capture" numbers from the surrounding scope
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}
