struct Animal {
    legs: u8,
    color: String,
}

impl Animal {
   fn new() -> Self {
    Self {
        legs: 4,
        color: String::from("black"),
    }
   }
}

fn main() {
    let mut dog = Animal::new();
    display(&dog);
    dog.color=String::from("brown");
    display(&dog);
    let cat = Animal {
        legs: 4,
        color: String::from("white"),
    };

    display(&cat);
}

fn display(animal : &Animal) {
    println!("Legs ={}, Color={}", animal.legs, animal.color);
}
