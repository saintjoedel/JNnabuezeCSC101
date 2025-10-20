use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter Your Height, (in centimetres):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let height: f32 = input.trim().parse().expect("Please enter a valid number");

    if height >= 150.0 && height <= 170.0 {
        println!("You are of average height");
    } else if height > 170.0 && height <= 195.0 {
        println!("You are tall");
    } else if height < 150.0 && height >= 100.0 {
        println!("You are short");
    } else {
        println!("Abnormal height");
    }
}