 fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Namee: {}", name );
    println!("University: {}, \nAddress: {}", uni,addr);

    let department:&'static str = "Computer Science and Technology";
    let school:&'static str = "School of Science amd Technology";
    println!("Department: {}, \nSchool: {}", department,school);
    
}