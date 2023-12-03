//Method to print the get value

fn value(n:Option<&char>){
    println!("Element of vector {:?}",n);
}

fn main(){

    let v = vec!['R','U','S','T','A','C','I','C','I','A','N'];

    let mut input1 = String::new();

    println!("\nEnter a index value btw (0 - 8)");
    std::io::stdin().read_line(&mut input1).expect("Invalid");
    let index:usize = input1.trim().parse().expect("Invalid");

    let ch:Option<&char> = v.get(index);
    value(ch);

}