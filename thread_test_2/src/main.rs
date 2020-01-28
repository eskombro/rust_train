use std::collections::HashMap;
use crossbeam::thread;
use std::sync::Mutex;

fn main() -> Result<(), std::io::Error> {
    let map = Mutex::new(HashMap::<String, String>::new());
    thread::scope(|s| {
        s.spawn(|_| {
            map.lock().unwrap().insert(
                "Thread_2".to_string(),
                "Some more shit".to_string(),
            );
            println!("A child thread borrowing `map`: {:?}", map);
        });
        s.spawn(|_| {
            map.lock().unwrap().insert(
                "Thread_1".to_string(),
                "Some more shit".to_string(),
            );
            println!("A child thread borrowing `map`: {:?}", map);
        });
    }).unwrap();
    let map = match map.into_inner() {
        Ok(map) => map,
        Err(err) => {
            println!("{:?}", err);
            err.into_inner()
        },
    };

    println!("{:?}",map);
    Ok(())
}
