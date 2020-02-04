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
    let vec: Vec::<Box<Animal>> = vec![Box::new(Horse), Box::new(Dog), Box::new(Cat)];

    println!("{} {}", vec.get(0).unwrap().do_noise(), vec.get(0).unwrap().eat());
    println!("{} {}", vec.get(1).unwrap().do_noise(), vec.get(1).unwrap().eat());
    println!("{} {}", vec.get(2).unwrap().do_noise(), vec.get(2).unwrap().eat());
}
