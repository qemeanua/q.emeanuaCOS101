use std::io;

fn main() {
    let mut input1 = String::new();
    let mut dob1 = String::new();
    let mut dob2 = String::new();
    let mut dob3 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut diagnosis = String::new();
    let mut residence = String::new();

    println!("The patient is required to fill in the following details: ");

    println!("\nName: ");
    io::stdin().read_line(&mut input1).expect("Not valid");

    println!("\nDate of Birth");

    println!("day: ");
    io::stdin().read_line(&mut dob1).expect("Not valid");

    println!("month: ");
    io::stdin().read_line(&mut dob2).expect("Not valid");

    println!("year of birth: ");
    io::stdin().read_line(&mut dob3).expect("Not valid");
    let year:f32 = dob3.trim().parse().expect("Not valid");
    
    println!("\nEmail Address: ");
    io::stdin().read_line(&mut input3).expect("Not valid");

    println!("\nPhone Number: ");
    io::stdin().read_line(&mut input4).expect("Not valid");

    println!("\nNumber of Siblings: "); 
    io::stdin().read_line(&mut input5).expect("Not valid");
    let siblings:f32 = input5.trim().parse().expect("Not valid");

    println!("\nNumber of Children: ");
    io::stdin().read_line(&mut input6).expect("Not valid");
    let no_of_children:f32 = input6.trim().parse().expect("Not valid");

    println!("\nMedical diagnosis: ");
    io::stdin().read_line(&mut diagnosis).expect("Not valid");
    //let diagnosis = input7.trim().parse().expect("Not valid");


    println!("\nVillage of residence: ");
    io::stdin().read_line(&mut residence).expect("Not valid");
    //let residence = input8.trim().parse().expect("Not valid");


    if year > 1975.0 &&  no_of_children > 4.0 && diagnosis == "alzheimer" && residence == "akpabom" {
        let p1:f32 = 12000000.0;
        let x1:f32 = 0.2 * p1;
        let d1:f32 = p1 - x1;

        println!(" Your charge is {}", d1 );

            }

    else if year == 1993.0 && siblings == 4.0 && residence == "ngbauji" && diagnosis == "arrythmia" {
        let p2:f32 = 550000.0;
        let x2:f32 = 0.05 * p2;
        let d2:f32 = p2 - x2;

        println!(" Your charge is {}", d2 );
    } 

   else  if year > 1983.0 && no_of_children > 3.0 && siblings > 3.0 && diagnosis == "ckd" && residence == "atabrikang" {
        let p3:f32 = 1500000.0;
        let x3:f32 = 0.15 * p3;
        let d3:f32 = p3 - x3;

        println!("Your charge is {} ", d3 );
    }

    else if year > 1995.0 && year < 1978.0 && diagnosis == "diabetes" && residence == "okorobilom" && no_of_children > 2.0 && no_of_children < 4.0 {
        let p4:f32 = 800000.0;
        let x4:f32 = 0.10 * p4;
        let d4:f32 = p4 - x4;

        println!("Your charge is {}", d4 );

    }  

   else if year > 1965.0 && no_of_children > 5.0 && siblings > 5.0 && diagnosis == "arthritis" && residence == "emeremen" {
        let p5:f32 = 450000.0;
        let x5:f32 = 0.10 * p5;
        let d5:f32 = p5 - x5;

        println!("Your charge is {}", d5 );
    }

    else {
            println!("Normal charges apply");
        }    
    }
