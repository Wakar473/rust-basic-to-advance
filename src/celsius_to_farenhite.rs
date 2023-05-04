use std::io;

pub fn celsius() {
    loop {
        println!("Select conversion type:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice != 1 && choice != 2 {
            println!("Invalid choice");
            continue;
        }

        println!("Enter temperature:");

        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let converted_temp = if choice == 1 {
            (temp - 32.0) * 5.0 / 9.0
        } else {
            (temp * 9.0 / 5.0) + 32.0
        };

        println!("Converted temperature: {:.2}", converted_temp);
        break;
    }
}
