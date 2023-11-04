fn main() {
   
   let fullname = "Emeanua Mazi Gold";
   let department = "Computer Science";
   let uni = "Babcock University";

   let mut school = "School of Science".to_string();
   //push string
   school.push_str("and Technology");

   println!("My name is {}", fullname );
   //check length
   println!("The length of my fullname is {}", fullname );
   println!("I am a student of {}  Department", department );
   println!("{}", school);
   println!("{}",uni );

}