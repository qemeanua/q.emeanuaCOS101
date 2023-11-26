fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of no is:{}",num );
}
 
 fn mutate_num_to_zero(mut paramum_num:i32) {
    paramum_num = paramum_num*0;
    println!("paramum_num value is : {}", paramum_num);

 }   
