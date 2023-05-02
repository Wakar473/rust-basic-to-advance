// In computer programming, a variable is a named storage location in memory that holds a value of 
// a specific data type, such as a number, a string, or a boolean. 
// The value stored in a variable can be changed or modified during the execution of a program, 
// making it a fundamental concept in computer programming.


//mutable variable
// pub fn mutable (){
//   let mut x = 1;
//   println!("no.x is :{x}");
//    let x= x+3;
//   println!("no. x is :{x}")  
// }


// find memory address of x
// you can print the memory address of a variable or a function using the "as" operator with a raw pointer type *const T or *mut T
//we print the memory address of x using the {:p}

pub fn mutable (){
    let x = 20;
    println!("no.x is :{x}");
     let x= x as *mut i32;
    println!("no. x is {:p}",x)  ;
  }
   

//shadowing
// pub fn mutable() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }