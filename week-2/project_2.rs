fn main(){
	let t:f64 = 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 750000.00;
	let d:f64 = 2850000.00;
	let a:f64 = 250000.00;

	let sum = (t*2.0) + (m*1.0) + (h*3.0) + (d*3.0) + (a*1.0);
	println!("The sum of product sales is: {}", sum);

	let average = sum/5.0;
	println!("The average of product sales is {}" , average);
}