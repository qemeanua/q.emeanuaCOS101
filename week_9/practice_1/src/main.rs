use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n ";
    let depth = "Department of Arts";

    let mut file = std::fs::File::create("data.txt").expect("Invalid");
    file.write_all("Welcome to Rust programming\n".as_bytes()).expect("Invalid");
    file.write_all(announce.as_bytes()).expect("Invalid");
    file.write_all(depth.as_bytes()).expect("Invalid");

    println!("\nData written to file.");
    
}