use ndarray::*;
use ndarray_example::add_arrays_ndarray;


fn main() {
  let a = Array1::from(vec![1.0, 2.0, 3.0]);
	let b = Array1::from(vec![4.0, 5.0, 6.0]);
	
	let result = add_arrays_ndarray(&a, &b);
	println!("ndarray result {:?}", result);
}