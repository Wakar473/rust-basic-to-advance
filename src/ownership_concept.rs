// pub fn owner(){
//         let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);

// }




// Pointer ‘Box’
// In Rust, a Box is a smart pointer that provides a way to allocate memory on the heap 
// and manage its lifetime. The Box type is defined in the Rust standard library and is used to allocate 
// memory on the heap and then store a value in that memory location.

// Box::new
// Box::new is a method in Rust's standard library that creates a new instance of the Box 
// smart pointer and allocates memory on the heap to hold a value of the specified type.

// fn main() {
//     let x = 5; // x is allocated on the stack
//     {
//         let y = 10; // y is also allocated on the stack
//     }
//     // y is out of scope and deallocated, x is still in scope
// }


// pub fn owner() {
//     let x = Box::new(5); // allocate memory on the heap
//     {
//         let y = Box::new(10); // allocate more memory on the heap
//         // y is still in scope, but x is also still in scope
//     }
//     // y is deallocated, but x is still in scope
//     // x is deallocated automatically when it goes out of scope
// }



// This line begins the declaration of the main function.
// This line creates a new String variable named s1 and initializes it with the value "hello".

// This line calls the calculate_length function with s1 
// as an argument, and assigns the return value of the function to a tuple containing s2 and len. 
// The tuple destructuring syntax is used to extract the two values 
// from the tuple returned by the calculate_length function.

// This line prints the values of s2 and len to the console using Rust's println!() macro. 
// The values are inserted into the output string using curly braces {} as placeholders.

// This line begins the declaration of the calculate_length function, 
// which takes a String argument named s and returns a tuple containing a String and a usize.

// This line creates a new variable named length and assigns it the length of the String s using the len() method.

// This line returns a tuple containing the original String s and its length as a usize. 
// The tuple is created implicitly using the comma , operator.
// This line ends the declaration of the calculate_length function.

// In summary, this code creates a String variable s1 containing the text "hello". 
// It then calls the calculate_length function, which takes in s1 and returns a tuple containing s1 and its length as a usize. 
// The tuple is destructured into s2 and len, which are printed to the console using the println!() macro.

// pub fn owner() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }



// pub fn owner (){
//     let bade= String::from("bhai");
// let chote=bade.clone();
// println!("bade and chote is {},{}",bade,chote);
// }




pub fn owner() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 10;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    print!("hey wakar {} take your,", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_int: i32) { // some_integer comes into scope
    println!("{}$", some_int);
} // Here, some_integer goes out of scope. Nothing special happens.