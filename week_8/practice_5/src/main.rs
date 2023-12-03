fn main() {
//Create an empty vector for city
    let mut city : Vec<String> = Vec::new();

    //print cty vector
    println!("The City vector has element {}",city.len());

    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Not valid");
    let city_num:i32 = input1.trim().parse().expect("Not valid");

    for count in 0..city_num{
        let mut input2 = String::new();
        println!("Enter City {}", count+1);
        std::io::stdin().read_line(&mut input2).expect("Not valid");
        let new_city:String = input2.trim().parse().expect("Not valid");
        city.push(new_city);
    }

    println!("Your preffered cities are: ");
   let mut count=1;

    for i in city{
        println!("{}", i );
    count+=1;
    }

    
}
