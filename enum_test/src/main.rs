enum Animal {
    Horse,
    Dog,
    Cat,
}

fn do_noise(animal: &Animal) -> String {
    match animal {
        Animal::Horse => String::from("A HORSE says BAHAHA"),
        Animal::Dog => String::from("A DOG says BARF"),
        Animal::Cat => String::from("A CAT says MIAOU"),
    }
}

fn eat(animal: &Animal) -> String {
    match animal {
        Animal::Horse => String::from("and eats CARROTS"),
        Animal::Dog => String::from("and eats ANYTHING"),
        Animal::Cat => String::from("and eats FISH"),
    }
}

fn main() {
    let zoo = vec![Animal::Horse, Animal::Dog, Animal::Cat];
    for animal in zoo {
        println!("{} {}", do_noise(&animal), eat(&animal));
    }
}
