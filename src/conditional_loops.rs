// Find  which number is Greater.

use std::io;

pub fn greater_number(){
    let  n:i32;
    println!("Please enter a number : -"); //0
    let mut a = String::new();// a = "0"
    io::stdin().read_line(& mut a).expect("not a valid"); //  
    n = a.trim().parse().expect("Not a Number"); // a = 0
    // let a = 0;

    if n>0{
        println!("a is +ve");

    }

    else if n<0{
        println!("a is -ve");
    }
    else  {
        println!("a is equal to zero");
    }
    // if (a>b){
    //     println!("A is Greater than B");
    
    // }
    // else if (a<b) {
    //     println!("B is Geater");
        
    // }
    // else{
    //     println!("Both are same");
        
    // }


     
    }


