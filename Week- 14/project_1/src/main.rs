use std::io;
use std::io::Read;

fn administator(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn project_manager(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn employee(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn customer(){
    let mut file = std::fs::File::open("customers_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn vendor(){
    let mut file = std::fs::File::open("dataplans_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn main() {

    println!("Welcome to Globacom Ltd
              Choose a number representing your level to access a database
              1 = Administator
              2 = Project Manager
              3 = Employee
              4 = Customer
              5 = Vendor");

let mut a =  String::new();

println!("Enter a number from 1 - 5 :");
io::stdin().read_line(&mut a ).expect("Invalid");
let  a1:i32 = a.trim().parse().expect("Invalid");

if a1  ==  1{
    administator();
}

else if a1 == 2 {
    project_manager();
}

else if a1 == 3{
    employee();
}

else if a1 == 4 {
    customer();
}

else if a1 == 5 {
    vendor();
}

else {
    println!("Couldn't access database
              Sorry!");
}
}
