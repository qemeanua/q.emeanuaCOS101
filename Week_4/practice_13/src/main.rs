 use std::io;

 fn main() {
    let mut input1 = String::new();
    let mut input2 =  String::new();
    let mut input3 = String::new();

    println!("Enter first of edge of triangle : ");
    io::stdin().read_line(&mut input1).expect("correct it");
    let a:f32 = input1.trim().parse().expect("not a valid number");

    println!("Enter second edge of triangle : ");
    io::stdin().read_line(&mut input2).expect("correct it");
    let b:f32 = input2.trim().parse().expect("not a valid number");

    println!("Enter third egde of triangle : ");
    io::stdin().read_line(&mut input3).expect("correct it");
    let c:f32 = input3.trim().parse().expect("not a valid string");

    let s:f32 = (a + b + c) / 2.0;
    let mut area:f32 = s * (s - a) * ( s * b) * ( s - c);
    area = area.sqrt();
    
    println!("Area of triangle: {}", area );
}