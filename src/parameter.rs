pub fn para() {
    print_labeled_measurement(5, 'h');
    print_name("wakar", 24,"kvs");
    print_company("antier" ,644);
    animal("king of the jungle", "amazon");
    print_human("peace", "heaven", 0);
    print("dell", "ubuntu");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

fn print_name(first_name:&str,age:i32,school:&str){
    println!("my details = {} age is {}, and school is {}", first_name,age,school);
}

fn print_company(company_name:&str ,emp_atr:i32){
    println!("company_name : {}, emp_atr:{}" , company_name, emp_atr);

}

fn animal(lion:&str ,place:&str){
    println!("lion : {}, {}",lion,place );
}

fn print_human(human:&str,place:&str,salary:i32){
    println!("human deserves {},place {},salary{}",human,place,salary);
}

fn print(system:&str ,os:&str){

println!("system is {},os is {}",system,os);
}