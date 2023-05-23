enum Fruit {
    Apple(String),
    Banana,
    Orange(i32),
}

// Each variant in the enum represents a distinct value. 
// Variants can optionally hold associated data.


pub fn enumeration() {
    let banana  = Fruit::Banana;
    let orange = Fruit::Orange(12);
    // let fruit = Fruit::Apple(String::from("Red Delicious"));

    match orange {
        Fruit::Apple(name) => println!("It's an apple: {}", name),
        Fruit::Banana => println!("It's a banana!"),
        Fruit::Orange(count) => println!("It's an orange with {} segments.", count),
    }
}
