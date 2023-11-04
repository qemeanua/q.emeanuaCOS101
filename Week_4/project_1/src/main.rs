fn main() {
    let distance_1:f64 = 80.0;
    let distance_2:f64 = 120.0;

    let d1_in_km = distance_1/1000.0;
    let d2_in_km = distance_2/1000.0;

    let t1:f64 = 2.0;
    let t2:f64 = 4.0;

    let s1 = d1_in_km/t1;
    let s2 = d2_in_km/t2;

    println!("The speed of the car in 2hours over a distance of 80 miles is {}km/h", s1);
    println!("\nThe speed of the car in 4hours over a distance of 120 miles is {}km/h", s2);  
}