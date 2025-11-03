use std::io;

fn main() {
    // Display menu
    println!("=== WELCOME TO OUR RESTAURANT ===");
    println!("P = Poundo Yam/Edinkaiko Soup   - N3,200");
    println!("F = Fried Rice & Chicken        - N3,000");
    println!("A = Amala & Ewedu Soup          - N2,500");
    println!("E = Eba & Egusi Soup            - N2,000");
    println!("W = White Rice & Stew           - N2,500");
    println!();

    // Get food choice
    let (food_name, price) = get_food_choice();

    // Get quantity
    let quantity = get_quantity();

    // Calculate total
    let subtotal = price * quantity;
    let discount = if subtotal > 10000 { subtotal / 20 } else { 0 };
    let final_amount = subtotal - discount;


    println!("\n ORDER SUMMARY ");
    println!("Food Item: {}", food_name);
    println!("Price: N{}", price);
    println!("Quantity: {}", quantity);
    println!("Subtotal: N{}", subtotal);
    
    if discount > 0 {
        println!("Discount: N{}", discount);
    }
    
    println!("Total: N{}", final_amount);
     println!("ENJOY YOUR MEAL");
}

fn get_food_choice() -> (String, u32) {
    loop {
        println!("Enter food choice (P, F, A, E, W):");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        match choice.trim().to_uppercase().as_str() {
            "P" => return ("Poundo Yam/Edinkaiko Soup".to_string(), 3200),
            "F" => return ("Fried Rice & Chicken".to_string(), 3000),
            "A" => return ("Amala & Ewedu Soup".to_string(), 2500),
            "E" => return ("Eba & Egusi Soup".to_string(), 2000),
            "W" => return ("White Rice & Stew".to_string(), 2500),
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn get_quantity() -> u32 {
    loop {
        println!("Enter quantity:");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read input");
        
        match quantity.trim().parse() {
            Ok(num) if num > 0 => return num,
            _ => println!("Please enter a valid number greater than 0"),
        }
    }

}