use std::io::Write;

fn main() {

    let student_name = vec!["Oluchi Mordi  ", "Adams Aliyu  ", "Shania Bolade ", "Adekunle Gold ", "Blanca Edemoh "];
    let matric_number = vec!["   ACC10211111    ", "    ECO10110101   ", "   CSC10328828    ", "   EEE11020202    ", "   MEE10202001    "];
    let department = vec!["Accounting ", " Economics  ", "Computer   ", "Electrical ", "Mechanical "];
    let level = vec![" 300", " 100", " 200", " 200", " 100"];
   

    let mut file = std::fs::File::create("SIMS.txt").expect("failed");
    file.write_all("                PAU SIMS  ".as_bytes()).expect("failed");
    file.write_all("\nStudent Name     Matric Number  Department  Level".as_bytes()).expect("failed");

    for i in 0..student_name.len(){
    file.write_all("\n".as_bytes()).expect("failed");
    file.write_all(  student_name[i].as_bytes()).expect("failed");

    file.write_all(  matric_number[i].as_bytes()).expect("failed");

    file.write_all(  department[i].as_bytes()).expect("failed");

    file.write_all(  level[i].as_bytes()).expect("failed");




    println!(" {} {} {} {}",student_name[i], matric_number[i], department[i], level[i]);   
}
  println!("\nFile created successfully");  
}

