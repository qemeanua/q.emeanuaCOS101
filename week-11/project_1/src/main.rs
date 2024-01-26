use std::io;    
use std::io::Write;

struct Company{
    name:String,
    shares:f32,
    liabilities:f32,
    constant:f32,
    //year:u32
}

impl Company{
    fn leverage(&self) ->f32{
      ( ( self.shares - self.liabilities) / self.shares ) * self.constant

    }
}

 fn main(){

     let co1  = Company{
        name:String::from("Cadbury Nigeria Plc"),
        shares:15000000.0,
        liabilities:5500000.0,
        constant: 100.0,
        //year: 1965
    };

    let co2  = Company{
        name:String::from("Champion Breweries Nigeria Plc"),
        shares:25000000.0,
        liabilities:8000000.0,
        constant: 100.0,
        //year: 1974
    };

    let co3  = Company{
        name:String::from("Dangote Sugar Nigeria Plc"),
        shares:18000000.0,
        liabilities:10000000.0,
        constant: 100.0,
        //year: 1970
    };

    let co4  = Company{
        name:String::from("Flour Mills Nigeria Plc"),
        shares:32000000.0,
        liabilities:4000000.0,
        constant: 100.0, 
        //year: 1960
    };

    let co5  = Company{
        name:String::from("Nestle Nigeria Plc"),
        shares:8000000.0,
        liabilities:1500000.0,
        constant: 100.0, 
        //year: 1961
    };

    let co6  = Company{
        name:String::from("Unilever Nigeria Plc"),
        shares:37000000.0,
        liabilities:11000000.0,
        constant: 100.0, 
        //year: 1923
    };

    let co7  = Company{
        name:String::from("Honeywell Nigeria Plc"),
        shares:34000000.0,
        liabilities:9000000.0,
        constant: 100.0,
       //year: 1906
    };

    let co8  = Company{
        name:String::from("Nigerian Breweries Plc"),
        shares:30000000.0,
        liabilities:12000000.0,
        constant: 100.0, 
        //year: 1946
    };

display(co1);
display(co2);
display(co3);
display(co4);
display(co5);
display(co6);
display(co7);
display(co8); 

let mut username = String::new();
let mut p1 = String::new();


println!("\nTo access the Companies full information you would have to input your username and password");
println!("\nEnsure that you input the right information needed");
println!("\nInput your username");
io::stdin().read_line(&mut username).expect("Invalid");
let username:String = username.trim().parse().expect("Invalid");

println!("\n Pick a password from the list below
    1. cad111
    2. cham112
    3. dag4113
    4. flo114
    5. nes115
    6. hon116
    7. nig117");
println!("\nInput the password you picked: ");
io::stdin().read_line(&mut p1).expect("Invalid");
let password = p1.trim();
let password_range = vec!["cad111", "cham112","dag113", "flo114" ,"nes115" ,"hon116" , "nig117"];

if password_range.contains(&password) && username.len() > 3 && username.len() < 9 {

let company_name = vec!["Cadbury Nigeria Plc              ","Champion Breweries Nigeria Plc   ","Dangote Sugar Refinery Plc       ","Flour Mills Nigeria Plc          ","Nestle Nigeria Plc               ","Unilever Nigeria Plc             ","Honeywell Nigeria Plc            ","Nigerian Breweries Plc           "];
let company_shares = vec!["15_000_000           ", "25_000_000           ", "18_000_000           ", "32_000_000           ", "8_000_000            ", "37_000_000           ", "34_000_000           ", "30_000_000           "];
let company_liabilities = vec!["5_500_000              ", "8_000_000              ", "10_000_000             ", "4_000_000              ", "1_500_000              ", "11_000_000             ", "9_000_000              ", "12_000_000             "];
let year_founded = vec!["1965  ", "1974  ", "1970  ", "1960  ", "1961  ", "1923  ", "1906  ", "1946  "];


let mut file = std::fs::File::create("COMPANY INFORMATION.txt").expect("failed");
    file.write_all("                COMPANY INFORMATION  ".as_bytes()).expect("failed");
    file.write_all("\nCompany Name                  Company Shares        Company liabilities     Year Founded".as_bytes()).expect("failed");

    for i in 0..company_name.len(){
    file.write_all("\n".as_bytes()).expect("failed");    
    file.write_all(  company_name[i].as_bytes()).expect("failed");
    file.write_all(  company_shares[i].as_bytes()).expect("failed");
    file.write_all(  company_liabilities[i].as_bytes()).expect("failed");
    file.write_all(  year_founded[i].as_bytes()).expect("failed");
}
println!("The file you want to access has been created.");
}

else {
    println!("Wrong username or password entered.");
}
}

fn display(co:Company){
    println!("\nThe percentage leverage of {} is {}  ", co.name, co.leverage(),);
}










