// Compound Types
// Compound types can group multiple values into one type.
//  Rust has two primitive compound types: tuples and arrays.

// The Tuple Type
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.


pub fn tuple() {
    let tup = (true, 6.4, 1,"human");

    let (_x, y, _z,_a) = tup;

    println!("The value of y is: {y}");
}


// Another way to have a collection of multiple values is with an array. Unlike a tuple,
//  every element of an array must have the same type. 
// Unlike arrays in some other languages, arrays in Rust have a fixed length.


pub fn array () {
    // Declare an array of integers with a length of 5
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Access elements of the array using their index
    println!("The first element of the array is {}", my_array[0]);
    println!("The second element of the array is {}", my_array[1]);

    // Iterate over the elements of the array using a for loop
    for i in 0..my_array.len() {
        println!("The element at index {} is {}", i, my_array[i]);
    }
}
