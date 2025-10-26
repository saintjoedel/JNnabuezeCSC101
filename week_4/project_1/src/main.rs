use std::io;

fn main() {
    let mut _a = String::new();
    let mut _b = String::new();
    let mut _c = String::new();

    println!("enter value for _a");
    io::stdin().read_line(&mut _a).expect("not valid");
    let _a: f64 = _a.trim().parse().expect("enter a new value");

    println!("enter value for _b");
    io::stdin().read_line(&mut _b).expect("not valid");
    let _b: f64 = _b.trim().parse().expect("enter a new value");
    
    println!("enter value for _c");
    io::stdin().read_line(&mut _c).expect("not valid");
    let _c: f64 = _c.trim().parse().expect("enter a new value");
    
    let _mut: f64 = _a * _c;
    let _f: f64 = _b * _b;
    let _e: f64 = 4.00 * _mut;
    let _g: f64 = _f - _e;
    
    if _g <= 0.00 {
        println!("equation has no real roots, enter new values");
    } else {
        let _h: f64 = _g.sqrt(); // calculates the square root
        let _t: f64 = 2.00 * _a;
        let _u: f64 = (-_b+_h) / _t;
        let _p: f64 = (-_b-_h) / _t;
        
        println!("positive root: (x + {}) = 0", _u);
        println!("negative root: (x + {}) = 0", _p); // Fixed: added + sign
    }
}
