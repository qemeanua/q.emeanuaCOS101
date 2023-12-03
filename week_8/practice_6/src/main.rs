fn main() {
    
    //Create two vectors
    let v = vec![1,2,3,4,5];
    let x = vec![10,11,12,13,14];

    //Use a for loop to add elements of the vector
    for index in 0..6{
        let sum = v[index] + x[index];
        println!("{:?}", sum );
    }
}
