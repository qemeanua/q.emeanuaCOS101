fn main() {

    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];

    let mut input1 = String::new();

    println!("Enter an index value btw (0 - 7");
    std::io::stdin().read_line(&mut input1).expect("Invalid");
    let index:usize = input1.trim().parse().expect("Invalid");

    let ch: char = v[index];

    print!("{} is the character for the index [{}]",ch,index );
       
}