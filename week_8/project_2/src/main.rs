fn main() {
  //Create  empty vector "years_of_programming experience"
  for _ in 1..5
  {
  println!("\nWelcome to EY Global limited
            We wish to you inform youy that you're one of our top 5 qualifiers
            Please enter in the following details to check whether you're eligble or not
            Goodluck!");  
  println!("Job interview for the best 5 applicants"); 
  //assuming the others didnt make the cut
  
  let mut  year_of_programming_experience : Vec<i32> = Vec::new();

  let mut applicants_name = String::new();
  let mut applicants_age = String::new();
  let mut c = String::new();

  println!("\nWhat is your name");
  std::io::stdin().read_line(&mut applicants_name).expect("Invalid");
  
  println!("\nWhat is your age");
  std::io::stdin().read_line(&mut applicants_age).expect("Invalid");
  

  println!("\nHow many years of programming do you have?");
  std::io::stdin().read_line(&mut c).expect("Invalid");
  let years:i32 = c.trim().parse().expect("Invalid");

   year_of_programming_experience.push(years);

   if years >= 7 {
  println!("\n {} you're eligle for the this job",applicants_name);
}

  else {
    println!("Oops sorry but you're not eligble for this job");
  }
 
 }
}
