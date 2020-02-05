enum Animal {
    Horse,
    Dog,
    Cat,
}

impl Animal {

    fn do_noise(&self) -> String {
        match self {
            Animal::Horse => String::from("A HORSE says BAHAHA"),
            Animal::Dog => String::from("A DOG says BARF"),
            Animal::Cat => String::from("A CAT says MIAOU"),
        }
    }

    fn eat(&self) -> String {
        match self {
            Animal::Horse => String::from("and eats CARROTS"),
            Animal::Dog => String::from("and eats ANYTHING"),
            Animal::Cat => String::from("and eats FISH"),
        }
    }
}

fn main() {
    let zoo = vec![Animal::Horse, Animal::Dog, Animal::Cat];
    for animal in zoo {
        println!("{} {}", animal.do_noise(), animal.eat());
    }
}
