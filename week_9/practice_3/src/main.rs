use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Ivalid");
    println!("fle is removed");
}
