fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	//compound interest
	let q:f64 = 1.0 + (r/100.0);
	let s:f64 = q.powf(n);
	let a = p * s; 
	println!("Amount is {}", a);
	let ci = a - p; 
	println!("Compound interest is {}" , ci);
	
}
