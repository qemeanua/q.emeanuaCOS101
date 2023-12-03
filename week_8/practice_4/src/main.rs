fn main() {

//Name vector

let name = vec!["Mary", "Josh", "Judah", "Ally"];

//Age vector
let age = vec![16,17,20,19];

println!("\nAge allocation");

for i in 0..age.len(){

    println!("{} is {} years old",name[i],age[i] );
}

}
