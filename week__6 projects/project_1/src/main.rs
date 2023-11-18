use std::io;

fn main() {

    for _ in 0..150{

    let mut name = String::new();
    let mut email = String::new();
    let mut department = String::new();
    let mut state_of_origin = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut current_class_rep = String::new();

    println!("\nYou're required to fill in the followig details");

    println!("\nName:");
    io::stdin().read_line(&mut name).expect("Not valid");

    println!("\nEmail:");
    io::stdin().read_line(&mut email).expect("Not valid");

    println!("\nDepartment:");
    io::stdin().read_line(&mut department).expect("Not valid");

    println!("\nState_of_origin");
    io::stdin().read_line(&mut state_of_origin).expect("Not valid");

    println!("\nCGPA:");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let cgpa:f32 = input1.trim().parse().expect("Not  valid");

    println!("\nYear level:");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let level:f32 = input2.trim().parse().expect("Not valid");

    println!("\nAre you a class rep:");
    io::stdin().read_line(&mut current_class_rep).expect("Not valid");

    let t = true; //type in t for true and f for false
    let f = false;
    let current_class_rep = t || f; //OR operation

    if current_class_rep == t && cgpa > 4.0 && level > 100.0
    {
        println!("\n{} {} {} {} you can vote", name, email, department, state_of_origin );
    }

       else {
           println!("\n{} {} {} {} you cannot vote", name, email, department, state_of_origin);
       }
    }   
}