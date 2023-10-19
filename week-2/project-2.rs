fn main(){
	let t:f64 = 2.0 * 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 3.0 * 750000.00;
	let d:f64 = 3.0 * 2850000.00;
	let a:f64 = 250000.00;

	let s = t + m + h + d + a;
	let avg = s / (2.0 + 1.0 + 3.0 + 3.0 + 1.0);

	println!("The sum is {}.", s);
	println!("The average is {}.", avg);
}