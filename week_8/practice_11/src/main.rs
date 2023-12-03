fn main() {
   //an array of numbers
   let numbers = [1 , 2, 3, 4, 5];
   println!("Origial array = {:?}", numbers );

   //create a slice of 2nd ad 3rd element
   let slice1 = &numbers[1..3];
   println!("2nd and 3rd elements sliced = {:?}", slice1 );

   //omit the start index
   let slice2 = &numbers[..3];
   println!("index  0 to index 3 sliced {:?}", slice2 );


   let slice3 = &numbers[2..];
   println!("index 2 to index 5 sliced {:?}", slice3 );

   let slice4 = &numbers[..];
   println!("index 0 to index 5 sliced = {:?}", slice4 );
}
