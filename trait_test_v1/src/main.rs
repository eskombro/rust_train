trait Animal {
    fn do_noise(&self) -> String;
    fn eat(&self) -> String;
}

struct Horse {
    data: AnimalData,
}

struct Dog {
    data: AnimalData,
}

struct Cat {
    data: AnimalData,
}

struct AnimalData {
    name: String,
    noise: String,
    food: String,
}

impl Animal for Horse {
    fn do_noise(&self) -> String {
        return format!("A {} says {}", self.data.name, self.data.noise);
    }
    fn eat(&self) -> String {
        return format!("and eats {}", self.data.food);
    }
}

impl Animal for Dog {
    fn do_noise(&self) -> String {
        return format!("A {} says {}", self.data.name, self.data.noise);
    }
    fn eat(&self) -> String {
        return format!("and eats {}", self.data.food);
    }
}

impl Animal for Cat {
    fn do_noise(&self) -> String {
        return format!("A {} says {}", self.data.name, self.data.noise);
    }
    fn eat(&self) -> String {
        return format!("and eats {}", self.data.food);
    }
}

fn main() {
    let h = Horse {
        data: AnimalData {
            name: String::from("Horse"),
            noise: String::from("Buahaha"),
            food: String::from("Carrots"),
        },
    };
    let d = Dog {
        data: AnimalData {
            name: String::from("Dog"),
            noise: String::from("Barf"),
            food: String::from("Anything"),
        },
    };
    let c = Cat {
        data: AnimalData {
            name: String::from("Cat"),
            noise: String::from("Miaou"),
            food: String::from("Fish"),
        },
    };
    println!("{} {}", h.do_noise(), h.eat());
    println!("{} {}", d.do_noise(), d.eat());
    println!("{} {}", c.do_noise(), c.eat());
}
