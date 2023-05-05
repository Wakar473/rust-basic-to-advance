pub fn clone(){
        let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

}

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


// fn main() {
//     let x = Box::new(5); // allocate memory on the heap
//     {
//         let y = Box::new(10); // allocate more memory on the heap
//         // y is still in scope, but x is also still in scope
//     }
//     // y is deallocated, but x is still in scope
//     // x is deallocated automatically when it goes out of scope
// }
