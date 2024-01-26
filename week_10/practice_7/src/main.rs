struct Employee {
 name:String,
 company:String,
 age:u32
}

fn main() {
    let emp1 = Employee {
        company:String::from("Enrst & Young"),
        name:String::from("Ebibiong Jessica"),
        age:25

    };

    println!("\nName = {}",emp1.name );
    println!("\nCompany = {}", emp1.company);
    println!("\nAge = {}", emp1.age);
    
}
