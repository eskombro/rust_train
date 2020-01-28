use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), std::io::Error> {
    let some_map = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    let map = some_map.clone();
    let thread_1 = thread::spawn(move || {
        let mut map1 = map.lock().unwrap();
        map1.insert(
            "Thread_1".to_string(),
            "Some shit".to_string(),
        );
    });

    let map = some_map.clone();
    let thread_2 = thread::spawn(move || {
        let mut map2 = map.lock().unwrap();
        map2.insert(
            "Thread_2".to_string(),
            "Some more shit".to_string(),
        );
    });
    thread_1.join().expect("Couldn't join on the associated thread");
    thread_2.join().expect("Couldn't join on the associated thread");
    println!("{:?}", some_map.lock().unwrap());
    Ok(())
}
