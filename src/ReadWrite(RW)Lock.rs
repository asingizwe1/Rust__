use std::sync::RwLock;
use std::thread;
fn main() {
    let config = RwLock::new("version 1".to_string());
    //constructs new RwLock<T> with string
    thread::scope(|s| {
        //scope()->e
        // Readers
        for _ in 0..3 {
            s.spawn(|| {
                let c = config.read().unwrap();
                println!("Reading config: {}", *c);
            });
        }

        // Writer
        s.spawn(|| {
            let mut c = config.write().unwrap();
            *c = "version 2".to_string();
            println!("Updated config");
        });
    });
}
