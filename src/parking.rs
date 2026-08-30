use std::collections::VecDeque; //VecDeque is used because it’s efficient for pushing/popping from both ends.
fn main() {
    let queue = Mutex::new(VecDeque::new()); //Protected by a Mutex so only one thread can modify it at a time.
    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item); //print item
            } else {
                thread::park();
            }
        });

        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
