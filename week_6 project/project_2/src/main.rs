use std::io;

 fn main() {
    for _ in 0..500{

        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("\nPlease fill in the following information: ");

        println!("\nName:");
        io::stdin().read_line(&mut input1).expect("Not valid");

        println!("\nNumber of papers published: ");
        io::stdin().read_line(&mut input2).expect("Not valid");
        let no_of_papers_published:i32 = input2.trim().parse().expect("Not valid");

        if no_of_papers_published >= 3 && no_of_papers_published <= 5 {
            println!("\nYour incentive is N500,000 ");
        }

        if no_of_papers_published >5 && no_of_papers_published < 10 {
            println!("\nYour incentive is N800,000 ");
        }

        else if no_of_papers_published >= 10 {
            println!("\nYour incentive is N1,000,000");
        }

        else if no_of_papers_published < 3 {
            println!("\nYour incentive is N100,000 ");
        }

        else {
            println!("\nNothing for you!");
        }
    }   
}