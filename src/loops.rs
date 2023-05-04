// Fortunately, Rust also provides a way to break out of a loop using code. 
// You can place the break keyword within the loop to tell the program when to stop executing the loop.

// pub fn loop_condition() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

//Use of a loop with a break statement to return a value from the loop. 
// Here's how the code works:

// The program starts by initializing a mutable variable counter to 0.

// The program then enters a loop block. Inside the loop, 
// the value of counter is incremented by 1 using the += operator.

// The program checks if the value of counter is equal to 10 using the == operator. 
// If the value of counter is 10, the program breaks out of the loop and returns the result of counter * 2 using the break statement.

// The program stores the value returned by the loop in the result variable.

// Finally, the program uses the println!() macro to print the value of the result variable.

// This will print the value of the result variable, 
// which should be 20 in this case, since the loop will break when counter is equal to 10, 
// and return counter * 2, which is 20.

// pub fn loop_condition() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

pub fn loop_condition() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 4 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}