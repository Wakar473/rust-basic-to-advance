pub fn test_no () {
    let mut test :i32 =101;
    if (test<=33 && test>=0){
        println!("student is fail")
    }
     else if (test<0){
        println!("number is not exist");
     }
    else if (test<=60 && test>=33){
        println!("student is 2nd division passed");
    }

    else if (test<=80 && test>60){
        println!("student is first division passed");
    }


    else if (test>80 && test <=100){
        print!("student is excellent");
    }
    else {
       println!("studnt belongs to bihar board")

    }
}