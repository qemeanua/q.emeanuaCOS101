fn main() {
    let v: Vec<i64> = Vec::new();

    //println size of vector
    println!("\nThe length of  Vec::new {}", v.len());

    //Usig macro
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    println!("\nThe length of the vec macro is : {}", v.len());
    
}