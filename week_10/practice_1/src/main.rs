fn main() {
   let v = vec![101, 250, 330, 400];
   //vector v owns the object in heap

   let _v2 =  &v;

   println!("{:?}",v);
}
