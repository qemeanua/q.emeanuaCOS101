use std::io;

 fn main() {

    let mut input_1 = String::new();
    let mut input_2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input_1).expect("Not a valid string");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input_2).expect("Not a valid string");
    let age:i32 = input_2.trim().parse().expect("Not a valid string");   

    if age >= 18 {
        println!("Welcomde to the party {}! ", input_1);
    } else {
        println!(" Oops, you're underaged {} ", input_1);
    } 
}