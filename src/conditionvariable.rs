use std::sync::{Mutex, Condvar};
use std::thread;

fn main() {
    let pair = (Mutex::new(None), Condvar::new());

    let (lock, cvar) = &pair;

    // Consumer thread
    let consumer = thread::spawn(move || {
        let mut data = lock.lock().unwrap();
        while data.is_none() {
            // Wait until producer sets a value
            data = cvar.wait(data).unwrap();
        }
        println!("Consumer got: {:?}", data.unwrap());
    });

    // Producer thread
    let (lock, cvar) = &pair;
    thread::spawn(move || {
        let mut data = lock.lock().unwrap();
        *data = Some(42); // set the value
        println!("Producer set value!");
        cvar.notify_one(); // wake up consumer
    });

    consumer.join().unwrap();
}
/*ared state: Mutex<Option<i32>> → starts as None.

Consumer: Locks the mutex, sees None, then calls wait().

wait() atomically unlocks the mutex and sleeps.

When woken, it relocks the mutex and checks again.

Producer: Locks the mutex, sets the value to Some(42), then calls notify_one().

Consumer wakes up: Sees the value, prints it. */

// use std::collections::VecDeque;
// use std::sync::{Condvar, Mutex};
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let queue = Mutex::new(VecDeque::new());
//     let not_empty = Condvar::new();

//     thread::scope(|s| {
//         // Consumer thread
//         s.spawn(|| {
//             loop {
//                 let mut q = queue.lock().unwrap();
//                 let item = loop {
//                     if let Some(item) = q.pop_front() {
//                         break item;
//                     } else {
//                         // Atomically unlocks mutex and waits
//                         q = not_empty.wait(q).unwrap();
//                     }
//                 };
//                 drop(q); // release lock before processing
//                 dbg!(item);
//             }
//         });

//         // Producer thread
//         for i in 0.. {
//             queue.lock().unwrap().push_back(i);
//             not_empty.notify_one(); // wake one waiting consumer
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// }
/**Consumers (miners): Multiple miners wait for new transactions.

Producer (network): Submits transactions into the mempool.

With condition variables, miners don’t need to be individually tracked. They all wait on the “mempool not empty” condition.

When a new transaction arrives, the network simply calls notify_one() (wake one miner) or notify_all() (wake all miners). */