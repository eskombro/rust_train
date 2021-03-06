trait Animal {
    fn do_noise(&self) -> String;
    fn eat(&self) -> String;
}

struct Horse;
struct Dog;
struct Cat;

impl Animal for Horse {
    fn do_noise(&self) -> String {
        String::from("A HORSE says BAHAHA")
    }
    fn eat(&self) -> String {
        String::from("and eats CARROTS")
    }
}

impl Animal for Dog {
    fn do_noise(&self) -> String {
        String::from("A DOG says BARF")
    }
    fn eat(&self) -> String {
        String::from("and eats ANYTHING")
    }
}

impl Animal for Cat {
    fn do_noise(&self) -> String {
        String::from("A CAT says MIAOU")
    }
    fn eat(&self) -> String {
        String::from("and eats FISH")
    }
}

fn main() {
    let h = Horse;
    let d = Dog;
    let c = Cat;

    println!("{} {}", h.do_noise(), h.eat());
    println!("{} {}", d.do_noise(), d.eat());
    println!("{} {}", c.do_noise(), c.eat());
}
