use std::io;

fn main() {
    
    let mut input1 = String::new();
    

    println!(" Enter age:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not a valid number");
   

    if age >= 40.0
    {
        println!("\nThe incentive of the experienced employee is N1_560_000");
    }

    else if age >= 30.0 && age < 40.0
    {
        println!("\nThe incentive of the experienced employee is N1_480_000");
    }

    else if age <30.0
    {
        println!("\nThe incentive of the experienced  employee is N1_300_000");
    }

    else {
        println!("\nYou're inxeperienced and you're incentive is N100_000");
    }


}