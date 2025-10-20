use std::io;
fn main() {
    println!("\nStudent Information Management System!");
    //input name
    println!("\nFull Name", );
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("invalided name"); 
    println!("Your name is:{}", name );
    //input age
    println!("\nage");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("invalided input");
    let age:i32 = age.trim().parse().expect("inpute not allowed");
    println!("Your age is: {}", age);
}
