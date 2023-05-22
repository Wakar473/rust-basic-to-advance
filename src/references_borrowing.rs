
pub fn borrow_reference(){
    
        let s1 = String::from("hello");
    
        let lambai = calculate_length(&s1);
    
        println!("The length of '{}' is {}.", s1, lambai);
    
    
    fn calculate_length(s: &String) ->usize {
       s.len()
    }
// Use of .clone();[DEEP COPY]
      
            let s1 = String::from("hello"); // declare a variable s1 and assign it a String
            let s2 = s1.clone(); // assign s1 to s2, transferring ownership of the String to s2
            // println!("{}", s1); // this line will cause a compile error since s1 is no longer the owner of the String
        
            println!("{}, {}", s2,s1); // prints "hello"

// use of refernece(&"am percent")
            let s1 = String::from("hello"); // declare a variable s1 and assign it a String
            let s2 = &s1; // assign s1 to s2, transferring ownership of the String to s2
            // println!("{}", s1); // this line will cause a compile error since s1 is no longer the owner of the String
        
            println!("{}{}", s2,s1); // prints "hello"

//ownership        
let s1 = String::from("hell"); // declare a variable s1 and assign it a String
            let s2 = s1; // assign s1 to s2, transferring ownership of the String to s2
            // println!("{}", s1); // this line will cause a compile error since s1 is no longer the owner of the String
        
            println!("{}", s2,); // prints "hello"

            // The key difference between deep copying and referencing is that a deep copy creates a 
            // completely independent copy of the data, while a reference just creates a pointer to the 
            // same data. In other words, when you modify a deep copy, the original data remains unchanged, 
            // while when you modify a reference, the original data is modified.




            //Borrowing
            //Ownership: str is an immutable string slice that can be borrowed from other string data, 
            // while String is an owned string that has full ownership over its data.

                let mut s = String::from("hello");
                let a = "amit";
                println!("{}",a);
                //change(s);
                change(&mut s);
            }
            
// some_string: It refers to a mutable String variable or a mutable String object.
// push_str(): It is a method provided by the String type in Rust. 
// The push_str() method appends the contents of the provided string slice (&str)
//  or String to the end of the existing string, modifying the original string.
               fn change(some_string: &mut String) {
                some_string.push_str(", wakar");
                println!("{}",some_string)
                
                
            
}


