extern crate almost_ord;
use almost_ord::AlmostOrdIterExt;
use almost_ord::AlmostOrdSliceExt;

#[test]
fn partial_max() {
	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	let max = vec.iter().partial_max().unwrap();
	assert_eq!(&5.0, max);
}

#[test]
fn vec_sort() {
	use std::f64;
	let mut vec = vec![5.0, 2.0, f64::INFINITY, 3.0, 5.0, 7.0, 27.0, f64::NAN, f64::NEG_INFINITY];
	//let mut vec = vec![5, 2, 3, 5];
	//vec.sort();
	vec.partial_sort();
	assert_eq!(&vec[0..vec.len()-1], &[f64::NEG_INFINITY, 2.0, 3.0, 5.0, 5.0, 7.0, 27.0, f64::INFINITY]);
}

#[test]
fn vec_binary_search() {
	let mut vec = vec![5.0, 2.0, 3.0, 5.0, 5.0, 5.0, 7.0, 27.0, std::f64::NAN];
	//let mut vec = vec![5, 2, 3, 5];
	//vec.sort();
	vec.partial_sort();
	//assert_eq!(&vec[0..4], &[2.0, 3.0, 5.0, 5.0]);
	assert_eq!(vec.partial_binary_search(&2.0), Ok(0));
	assert_eq!(vec.partial_binary_search(&3.0), Ok(1));

	let idx = vec.partial_binary_search(&5.0);
	assert!((2..6).any(|n| Ok(n) == idx));
}
