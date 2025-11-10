use std::io;

fn main() {
    loop {
        println!("1. Trapezium Area");
        println!("2. Rhombus Area");
        println!("3. Parallelogram Area");
        println!("4. Cube Area");
        println!("5. Cylinder Volume");
        println!("q. Quit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => trapezium_area(),
            "2" => rhombus_area(),
            "3" => parallelogram_area(),
            "4" => cube_area(),
            "5" => cylinder_volume(),
            "q" => break,
            _ => continue,
        }
    }
}

fn trapezium_area() {
    let h = get_num("Height:");
    let b1 = get_num("Base1:");
    let b2 = get_num("Base2:");
    let area = (h / 2.0) * (b1 + b2);
    println!("Area: {:.2}", area);
}

fn rhombus_area() {
    let d1 = get_num("Diagonal1:");
    let d2 = get_num("Diagonal2:");
    let area = 0.5 * d1 * d2;
    println!("Area: {:.2}", area);
}

fn parallelogram_area() {
    let base = get_num("Base:");
    let height = get_num("Height:");
    let area = base * height;
    println!("Area: {:.2}", area);
}

fn cube_area() {
    let side = get_num("Side:");
    let area = 6.0 * side * side;
    println!("Area: {:.2}", area);
}

fn cylinder_volume() {
    let r = get_num("Radius:");
    let h = get_num("Height:");
    let volume = 3.14159 * r * r * h;
    println!("Volume: {:.2}", volume);
}

fn get_num(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse() {
            return num;
        }
    }
}