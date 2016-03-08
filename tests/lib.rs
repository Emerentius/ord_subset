extern crate ord_subset;
use ord_subset::OrdSubsetIterExt;
use ord_subset::OrdSubsetSliceExt;

#[test]
fn ord_subset_max() {
	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	let max = vec.iter().ord_subset_max().unwrap();
	assert_eq!(&5.0, max);
}

#[test]
fn ord_subset_max_by() {
	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	let max_by = vec.iter().ord_subset_max_by_key(|num| num.recip()).unwrap();
	assert_eq!(&2.0, max_by);
}

#[test]
fn ord_subset_min() {
	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	let min = vec.iter().ord_subset_min().unwrap();
	assert_eq!(&2.0, min);
}

#[test]
fn ord_subset_min_by() {
	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	let min_by = vec.iter().ord_subset_min_by_key(|num| num.recip()).unwrap();
	assert_eq!(&5.0, min_by);
}

#[test]
fn vec_sort() {
	use std::f64;
	let mut vec = vec![5.0, 2.0, f64::INFINITY, 3.0, 5.0, 7.0, 27.0, f64::NAN, f64::NEG_INFINITY];
	//let mut vec = vec![5, 2, 3, 5];
	//vec.sort();
	vec.ord_subset_sort();
	assert_eq!(&vec[0..vec.len()-1], &[f64::NEG_INFINITY, 2.0, 3.0, 5.0, 5.0, 7.0, 27.0, f64::INFINITY]);
}

#[test]
fn vec_binary_search() {
	use std::f64;
	let mut vec = vec![5.0, 2.0, 3.0, 5.0, 5.0, 5.0, 7.0, 27.0, f64::NAN];
	//let mut vec = vec![5, 2, 3, 5];
	//vec.sort();
	vec.ord_subset_sort();
	//assert_eq!(&vec[0..4], &[2.0, 3.0, 5.0, 5.0]);
	assert_eq!(vec.ord_subset_binary_search(&2.0), Ok(0));
	assert_eq!(vec.ord_subset_binary_search(&3.0), Ok(1));

	let idx = vec.ord_subset_binary_search(&5.0);
	assert!((2..6).any(|n| Ok(n) == idx));
}


#[test]
fn vec_rev_sort() {
	use std::f64;
	let mut vec = vec![5.0, 2.0, f64::INFINITY, 3.0, 5.0, 7.0, 27.0, f64::NAN, f64::NEG_INFINITY];
	vec.ord_subset_sort_rev();
	assert_eq!(&vec[0..vec.len()-1], &[f64::INFINITY, 27.0, 7.0, 5.0, 5.0, 3.0, 2.0, f64::NEG_INFINITY]);
}

#[test]
fn vec_rev_binary_search() {
	use std::f64;
	let mut vec = vec![5.0, 2.0, f64::INFINITY, 3.0, 5.0, 7.0, 27.0, f64::NAN, f64::NEG_INFINITY];
	vec.ord_subset_sort_rev();
	assert_eq!(&vec[0..vec.len()-1], &[f64::INFINITY, 27.0, 7.0, 5.0, 5.0, 3.0, 2.0, f64::NEG_INFINITY]);
	assert_eq!(vec.ord_subset_binary_search_rev(&f64::NEG_INFINITY), Ok(7));
	assert_eq!(vec.ord_subset_binary_search_rev(&2.0), Ok(6));
	assert_eq!(vec.ord_subset_binary_search_rev(&3.0), Ok(5));
	let idx = vec.ord_subset_binary_search_rev(&5.0); // duplicate
	assert!((3..5).any(|n| Ok(n) == idx));
	assert_eq!(vec.ord_subset_binary_search_rev(&7.0), Ok(2));
	assert_eq!(vec.ord_subset_binary_search_rev(&27.0), Ok(1));
	assert_eq!(vec.ord_subset_binary_search_rev(&f64::INFINITY), Ok(0));
}


#[test]
fn array_rev_sort() {
	use std::f64;

	let mut s  = [0., 1., f64::NAN, 1., 1., 1., 2., 3., 5., 8., 13., 21., 34., 55., f64::NAN];
	let s2     = [55., 34., 21., 13., 8., 5., 3., 2., 1., 1., 1., 1., 0., f64::NAN, f64::NAN];
	s.ord_subset_sort_rev();
	assert_eq!(&s[..s.len()-2], &s2[..s2.len()-2]);
}

#[test]
fn array_rev_sort_by() {
	use std::f64;

	let mut s  = [0., 1., f64::NAN, 1., 1., 1., 2., 3., 5., 8., 13., 21., 34., 55., f64::NAN];
	let s2     = [55., 34., 21., 13., 8., 5., 3., 2., 1., 1., 1., 1., 0., f64::NAN, f64::NAN];
	s.ord_subset_sort_by(|a,b| b.partial_cmp(a).unwrap());
	assert_eq!(&s[..s.len()-2], &s2[..s2.len()-2]);
}

// the equivalent reverse is in the docs for ord_subset_binary_search()
#[test]
fn array_rev_binary_search_with_nan() {
	use std::f64;

	let s = [55., 34., 21., 13., 8., 5., 3., 2., 1., 1., 1., 1., 0., f64::NAN, f64::NAN];

	assert_eq!(s.ord_subset_binary_search_rev(&13.),  Ok(3));
	assert_eq!(s.ord_subset_binary_search_rev(&4.),   Err(6));
	assert_eq!(s.ord_subset_binary_search_rev(&100.), Err(0));
	let r = s.ord_subset_binary_search_rev(&1.);
	assert!(match r { Ok(8...11) => true, _ => false, });
	assert_eq!(s.ord_subset_binary_search_rev(&f64::INFINITY), Err(0));
	assert_eq!(s.ord_subset_binary_search_rev(&f64::NEG_INFINITY), Err(13));
}
