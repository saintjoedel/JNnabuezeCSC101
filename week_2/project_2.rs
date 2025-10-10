fn main(){


    let a = 450000.00; //this variable stores the price of toshiba
    let b = 1500000.00; //this variable stores the price of mac
    let c = 750000.00; //this variable stores the price of hp
    let d = 2850000.00;//this variable stores the price of dell
    let e = 250000.00;//this variable stores the price of acer
    let h = 2.00; //this variable stores the number of toshiba
    let i = 1.00;//this variable stores the number of mac
    let j = 3.00;//this variable stores the number of hp
    let k = 3.00;//this variable stores the number of dell
    let l = 1.00;//this variable stores the number of acer
let sum = a*h+b+c*j+d*k+e;
let x = h+i+j+k+l;
let average = sum/x;
println!("sum {}",sum);
println!("average {}", average);
}