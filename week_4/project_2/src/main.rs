use std::io;

fn main() {
    println!("Employee Annual Incentive");
    let experience = get_experience_input();
    let age = get_age_input();
    let incentive = calculate_incentive(experience, age);
    println!("\nAnnual Incentive");
    println!("Employee Status: {}", if experience { "Experienced" } else { "Inexperienced" });
    println!("Age: {} years", age);
    println!("Annual Incentive: N{}", incentive);
}
fn get_experience_input() -> bool {
    loop {
        println!("\nIs the employee experienced? (yes/no):");
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("invailed data");
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no'"),
        }
    }
}

fn get_age_input() -> u32 {
    loop {
        println!("\nEnter employee's age:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("invailed data");
            
        match input.trim().parse() {
            Ok(age) if age > 0 && age <= 75 => return age,
            Ok(_) => println!("Please enter age (1-75)"),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

fn calculate_incentive(experience: bool, age: u32) -> u32 {
    if !experience {
        // Inexperienced employee
        100_000
    } else {
        // Experienced employees
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age < 40 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            println!("Age {} falls between explicitly defined ranges. Using base experienced rate.", age);
            1_300_000
        }
    }
}