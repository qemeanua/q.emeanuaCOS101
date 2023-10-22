fn main() {

let p:f64 = 210000.0;
let r:f64 = 5.0;
let n:f64 = 3.0;

let s = 1.0 - (r/100.0);
let b:f64 = s.powf(n);
let a = p * b;
println!("The amount is equal to {}" , a);

let ci = a - p;
println!("The compound interest is {}" , ci);
}
