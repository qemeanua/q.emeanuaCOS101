use std::io;

fn main(){

    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();


    println!("enter value a: ");
    io::stdin().read_line(&mut input_1).expect("Not valid");
    let a:f32 = input_1.trim().parse().expect("Not a valid number");

    println!("enter value b: ");
    io::stdin().read_line(&mut input_2).expect("Not valid");
    let b:f32 = input_2.trim().parse().expect("Not a valid number");

    println!("enter value c: ");
    io::stdin().read_line(&mut input_3).expect("Not valid");
    let c:f32 = input_3.trim().parse().expect("Not a valid number");

    let d = b*b  + (4.0*(a*c));
    println!("The discriminant is {}", d);

    if d >0.0
    {
        let sqrt_d = d.sqrt();
        let root_1 = (-b + sqrt_d )/ ( 2.0 * a);
        let root_2 = (-b - sqrt_d ) / ( 2.0 * a);
        println!("The two real roots are {} {} ", root_1 , root_2);
    }

    else if d == 0.0
    {
        println!("There is exactly one real root.");
    }

    else if d < 0.0
    {
        println!("There are no real roots.");

    }

    else {
        println!("Check your equation.")
    }





}

