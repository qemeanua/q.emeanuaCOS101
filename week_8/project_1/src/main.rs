fn office_adminstator(){

    let mut level : Vec<String> = Vec::new();

    let mut level_ = String::new();
    

    println!("Which Office Administation level do you belong to?");
    std::io::stdin().read_line(&mut level_).expect("Invalid");
    let level_: String = level_.trim().parse().expect("Invalid");

     level.push(level_.clone());

    if level_ == "Intern"{
    println!("Your Public Service APS level is APS 1-2 "); }

    else if level_ == "Administator"{
        println!("Your Public Service APS level is APS 3-5");
    }

    else if level_ == "Senior Administator"{
        println!("Your Public Service APS level is APS 5-8");
    }

    else if level_ == "Office manager"{
        println!("Your Public Service APS level is EL1 8-10");
    }

    else if level_ == "Director"{
        println!("Your Public Service APS level is EL2 10-13 ");
    }

    else if level_ == "Ceo"{
        println!("Your Public Service APS level is SES");
    }

    else {
        println!("Couldn't find APS level");
    }
} 

fn academic(){
    let mut level : Vec<String> = Vec::new();

    let mut level_ = String::new();

     println!("\nWhich academic level do you belong to?");
    std::io::stdin().read_line(&mut level_).expect("Invalid");

    level.push(level_.clone());

     if level_ == "Research Assistant" {
        println!("Your Public Service APS level is APS 3-5");
     }

     else if level_ == "PhD Candidate" {
        println!("Your Public Service APS level is APS 5-8");
     }

     else if level_ == "Post-Doc Researcher" {
        println!("Your Public Service APS level is EL1 8-10");
     }

     else if level_ == "Senior Lecturer" {
        println!("Your Public Service APS level is EL2 10-13");
     }

     else if level_ == "Dean" {
        println!("Your Public Service APS level is SES");
     }

     else {
         println!("Couldn't find APS level");
     }
}

fn lawyer(){
    let mut level : Vec<String> = Vec::new();

    let mut level_ = String::new();

    println!("What law level do you belong to?");
    std::io::stdin().read_line(&mut level_).expect("Invalid");

    level.push(level_.clone());

    if level_ == "Paralegal"{
        println!("Your Public Service APS level is APS 1-2");
    } 

    if level_ == "Junior Associate"{
        println!("Your Public Service APS level is APS 3-5");
    }

    if level_ == "Associate"{
        println!("Your Public Service APS level is APS 5-8");
    }

    if level_ == "Senior Associate 1-2"{
        println!("Your Public Service APS level is EL1 8-10");
    }

    if level_ == "Senior Associate 3-4"{
        println!("Your Public Service APS level is EL2 10-13");
    }

    if level_ == "Partner"{
        println!("Your Public Service APS level is SES");
    }

    else {
        println!("Couldn't find APS level");
    }
}

fn teacher(){
    let mut level : Vec<String> = Vec::new();

    let mut level_ = String::new();

    println!("What teaching level do you belong to?");
    std::io::stdin().read_line(&mut level_).expect("Invalid");

    level.push(level_.clone());

    if level_ == "Placement"{
        println!("Your Public Service APS level is APS 1-2");
    } 

    if level_ == "Classroom Teacher"{
        println!("Your Public Service APS level is APS 3-5");
    }

    if level_ == "Snr Teacher"{
        println!("Your Public Service APS level is APS 5-8");
    }

    if level_ == "Leading Teacher"{
        println!("Your Public Service APS level is EL1 8-10");
    }

    if level_ == "Deputy Principal"{
        println!("Your Public Service APS level is EL2 10-13");
    }

    if level_ == "Principal"{
        println!("Your Public Service APS level is SES");
    }

    else {
        println!("Couldn't find APS level");
    }

}

 fn main() {

    let mut choice : Vec<i32> = Vec::new();

    let mut input1 = String::new();

    println!("\nWelcome to the Public Service APS level checker.
             The numbers below represent a department in the Public Service.
              1 = Office Administator
              2 = Academic
              3 = Lawyer
              4 = Teacher");

    println!("\nWhich number do you choose?");
    std::io::stdin().read_line(&mut input1 ).expect("Invalid");
    let choice_:i32 = input1.trim().parse().expect("Invalid");

    choice.push(choice_);

    if choice_ == 1 {
        
        office_adminstator();
    }

    if choice_ == 2 {
        academic();
    }

    if choice_ == 3 {
        lawyer();
    }

    if choice_ == 4 {
        teacher();
    }

    else {
        println!("\nYou do not belong to the Public Service!");
    }
    
}