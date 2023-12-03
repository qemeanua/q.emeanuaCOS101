fn main() {
   //intialize mutable tuple

   let mut mountain_heights = ("Everest", 8848 , "Fishtail", 9087);

   println!("Orinal tuple = {:?}", mountain_heights );

   mountain_heights.2 = "Lagos";
   mountain_heights.3 = 7790;

   println!("Changed tuple = {:?}", mountain_heights );
}
