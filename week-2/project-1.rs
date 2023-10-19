fn main(){
	let p:f64 = 520000000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	let a = p * ((1.0 + (r / 100.0)).powf(n));
	let c = a - p;

	println!("The compound interest is {}.", c);

}