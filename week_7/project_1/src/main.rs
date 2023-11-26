use std::io;

fn area_of_trapezium_formula(){
 let mut a = String::new();
 let mut b = String::new();
 let mut c = String::new();

 println!("Enter the height of the trapezium:");
 io::stdin().read_line(&mut a).expect("Invalid");
 let h:f32 = a.trim().parse().expect("Invalid");

 println!("Enter base 1 of the trapezium:");
 io::stdin().read_line(&mut b).expect("Invalid");
 let b1:f32 = b.trim().parse().expect("Invalid");

 println!("Enter base 2 of the trapezium:");
 io::stdin().read_line(&mut c).expect("Invalid");
 let b2:f32 = c.trim().parse().expect("Invalid");

 let area = (h/2.0) * (b1 + b2);
 println!("The area of the trapezium is {} ", area );
}

fn area_of_rhombus_formula(){
    let mut p = String::new();
    let mut q = String::new();

 println!("Enter diagonal1 of the rhombus:");
 io::stdin().read_line(&mut p).expect("Invalid");
 let d1:f32 = p.trim().parse().expect("Invalid");

 println!("Enter diagonal1 of the rhombus:");
 io::stdin().read_line(&mut q).expect("Invalid");
 let d2:f32 = q.trim().parse().expect("Invalid");

 let area2 = 0.5 * (d1 * d2);
 println!("The area of the rhombus is {}", area2 );
}

fn area_of_parallelogram_formula(){
    let mut f = String::new();
    let mut g = String::new();

println!("Enter base of the parallelogram:");
 io::stdin().read_line(&mut f).expect("Invalid");
 let b3:f32 = f.trim().parse().expect("Invalid");

 println!("Enter altitude of the parallelogram:");
 io::stdin().read_line(&mut g).expect("Invalid");
 let a1:f32 = g.trim().parse().expect("Invalid");

 let area3 = b3 * a1;
 println!("The area of the parallelogram is {}", area3);
}

fn cube_formula(){
    let mut x = String::new();

 println!("Enter lenght of side of the cube:");
 io::stdin().read_line(&mut x).expect("Invalid");
 let l1:f32 = x.trim().parse().expect("Invalid");

 let area4 = 6.0 * (l1 * l1);
 println!("The area of the cube is {}", area4);
}

fn volume_of_cylinder_formula(){
    let mut y = String::new();
    let mut z = String::new();

    println!("Enter radius of the cylinder:");
 io::stdin().read_line(&mut y).expect("Invalid");
 let r:f32 = y.trim().parse().expect("Invalid");

println!("Enter height of the cylinder:");
 io::stdin().read_line(&mut z).expect("Invalid");
 let h:f32 = z.trim().parse().expect("Invalid");

 let area5:f32 = 3.142 * ( r * r ) * h;
 println!("The volume of the cylinder is {}", area5); 
}

fn main(){

    println!("\nChoose the calculation to be performed");
    println!("1. Area of trapezium
              2. Area of rhombus
              3. Area of parallelogram
              4. Area of cube
              5. Volume of cylinder");

 let mut input1 = String::new();
 println!("Which calculation do you want to perform:");
 io::stdin().read_line(&mut input1).expect("Invalid");
 let choice:i32 = input1.trim().parse().expect("Invalid");

if choice == 1 {
    area_of_trapezium_formula();
}

else if choice == 2{
    area_of_rhombus_formula();
}

else if choice == 3 {
    area_of_parallelogram_formula();
}

else if choice == 4 {
    cube_formula();

}

else if choice == 5 {
    volume_of_cylinder_formula();
}

else {
    println!("error");
}
}