use std::collections::HashSet;

fn main() {
    let some_str = String::from("blah blah2 blah3");
    let some_str = some_str.split(" ");
    let mut set = HashSet::<&str>::new();

    for item in some_str {
        set.insert(&item);
    }
    println!("{:?}", set);
}
