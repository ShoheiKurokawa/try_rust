// This is a simple example of a trait in Rust
trait Animal {
    fn speak(&self);
    fn sleep_time(&self);
    fn speed(&self);
    fn age(&self);
}

struct Dog {
    name: String,
    age: u8,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says Woof!", self.name); // Woof!
    }

    fn sleep_time(&self) {
        println!("{} sleeps at 9:00 pm", self.name); // 9:00 pm
    }

    fn speed(&self) {
        println!("{} runs at 20 km/h", self.name);
    }

    fn age(&self) {
        println!("{} is {} years old", self.name, self.age);
    }
}

struct Cat {
    name: String,
    age: u8,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says Meow!", self.name); // Meow!
    }

    fn sleep_time(&self) {
        println!("{} sleeps at 10:00 pm", self.name); // 10:00 pm
    }

    fn speed(&self) {
        println!("{} runs at 15 km/h", self.name);
    }

    fn age(&self) {
        println!("{} is {} years old", self.name, self.age);
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
        age: 5,
    };

    let cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };

    dog.speak();
    dog.sleep_time();
    dog.speed();
    dog.age();

    cat.speak();
    cat.sleep_time();
    cat.speed();
    cat.age();
}
