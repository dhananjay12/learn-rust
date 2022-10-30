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

trait Speak {
   fn speak(&self) -> &str;
}

impl Speak for Animal {
    fn speak(&self) -> &str {
        "Grr"
    }
}

fn print_speak<T: Speak> (item: T){
    println!("{}", item.speak());
}

fn main() {
    let dog = Animal::new();
    display(&dog);
    print_speak(dog);
}

fn display(animal : &Animal) {
    println!("Legs ={}, Color={}", animal.legs, animal.color);
}
