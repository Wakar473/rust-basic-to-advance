
pub fn borrow(){
    
        let s1 = String::from("hello");
    
        let lambai = calculate_length(&s1);
    
        println!("The length of '{}' is {}.", s1, lambai);
    
    
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
// Use of .clone();[DEEP COPY]
      
            let s1 = String::from("hello"); // declare a variable s1 and assign it a String
            let s2 = s1.clone(); // assign s1 to s2, transferring ownership of the String to s2
            // println!("{}", s1); // this line will cause a compile error since s1 is no longer the owner of the String
        
            println!("{}{}", s2,s1); // prints "hello"

// use of refernece(&"am percent")
            let s1 = String::from("hello"); // declare a variable s1 and assign it a String
            let s2 = &s1; // assign s1 to s2, transferring ownership of the String to s2
            // println!("{}", s1); // this line will cause a compile error since s1 is no longer the owner of the String
        
            println!("{}{}", s2,s1); // prints "hello"

//ownership        
let s1 = String::from("hello"); // declare a variable s1 and assign it a String
            let s2 = s1; // assign s1 to s2, transferring ownership of the String to s2
            // println!("{}", s1); // this line will cause a compile error since s1 is no longer the owner of the String
        
            println!("{}", s2,); // prints "hello"

        
}
