extern crate ord_subset;
extern crate core;
use ord_subset::OrdSubsetIterExt;
use ord_subset::OrdSubsetSliceExt;
use ord_subset::OrdSubset;
use ord_subset::OrdVar;

use std::f64::INFINITY as INF;
use std::f64::NAN;

const N: usize = 32;
const N_NO_NAN: usize = 30;

const TEST_ARRAY: [f64; N] =
	[1.0,  7.0,  26.0,  INF,  NAN,  0.0, 14.0, 17.0,
	27.0, 13.0,  10.0,  3.0,  NAN, 25.0,  9.0, 20.0,
	16.0,  8.0,  -INF,  4.0,  2.0, 22.0, 18.0, 21.0,
	15.0,  6.0,  24.0, 19.0, 12.0, 11.0,  5.0, 23.0];

// subset of TEST_ARRAY
const TEST_ARRAY_NO_NAN: [f64; N_NO_NAN] =
	[1.0,  7.0,  26.0,  INF,        0.0, 14.0, 17.0,
	27.0, 13.0,  10.0,  3.0,       25.0,  9.0, 20.0,
	16.0,  8.0,  -INF,  4.0,  2.0, 22.0, 18.0, 21.0,
	15.0,  6.0,  24.0, 19.0, 12.0, 11.0,  5.0, 23.0];

const SORTED_TEST_ARRAY: [f64; N] =
	[-INF, 0.0,  1.0,  2.0,  3.0,  4.0,  5.0,  6.0,
	 7.0,  8.0,  9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
	15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0,
	23.0, 24.0, 25.0, 26.0, 27.0,  INF,  NAN,  NAN];

const SORTED_TEST_ARRAY_NO_NAN: [f64; N_NO_NAN] =
	[-INF, 0.0,  1.0,  2.0,  3.0,  4.0,  5.0,  6.0,
	 7.0,  8.0,  9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
	15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0,
	23.0, 24.0, 25.0, 26.0, 27.0, INF             ];


#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct NotOrdSub();
#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct OrdSub();

impl OrdSubset for OrdSub {
	fn is_outside_order(&self) -> bool {
		true
	}
}

// ---------------------------- iter ext methods -------------------------------

#[test]
fn ord_subset_max() {
	let arr = [2.0, 3.0, 5.0, std::f64::NAN];
	let max = arr.iter().ord_subset_max().unwrap();
	assert_eq!(&5.0, max);
}

#[test]
fn ord_subset_max_by() {
	let arr = [2.0, 3.0, 5.0, std::f64::NAN];
	let max_by = arr.iter().ord_subset_max_by_key(|num| num.recip()).unwrap();
	assert_eq!(&2.0, max_by);
}

#[test]
fn ord_subset_min() {
	let arr = [2.0, 3.0, 5.0, std::f64::NAN];
	let min = arr.iter().ord_subset_min().unwrap();
	assert_eq!(&2.0, min);
}

#[test]
fn ord_subset_min_by() {
	let arr = [2.0, 3.0, 5.0, std::f64::NAN];
	let min_by = arr.iter().ord_subset_min_by_key(|num| num.recip()).unwrap();
	assert_eq!(&5.0, min_by);
}

// This is a compile time test. It can't fail at runtime.
// The referenced functions must accept iters of values, that are not OrdSubset
// if the closure produces OrdSubset values
#[allow(unused)]
fn ord_subset_min_or_max_by_key() {
	let array: [NotOrdSub; 0] = [];
	array.iter().ord_subset_min_by_key(|_| 0.0);
	array.iter().ord_subset_max_by_key(|_| 0.0);
}

// ---------------------------slice ext methods --------------------------------
// ----------------------------- stable sorts ----------------------------------

#[test]
#[cfg(feature="std")]
fn sort() {
	let mut array = TEST_ARRAY;
	array.ord_subset_sort();
	assert_eq!(&array[0..N_NO_NAN], &SORTED_TEST_ARRAY_NO_NAN);
}

#[test]
#[cfg(feature="std")]
fn sort_rev() {
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_rev();

	let mut rev_sorted_array = SORTED_TEST_ARRAY_NO_NAN;
	rev_sorted_array.reverse();

	assert_eq!(&array[0..N_NO_NAN], &rev_sorted_array);
}

#[test]
#[cfg(feature="std")]
fn sort_by_key() {
	fn key_function(el: &f64) -> f64 {
		(el - 13.0).recip()
	}
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_by_key(key_function);
	let mut std_sorted_array = TEST_ARRAY_NO_NAN;
	std_sorted_array.sort_by_key(|num| OrdVar::new(key_function(num)));
	assert_eq!(&array[..N_NO_NAN], &std_sorted_array);
}

// ----------------------------- unstable sorts --------------------------------

#[test]
fn sort_unstable() {
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_unstable();
	assert_eq!(&array[0..N_NO_NAN], &SORTED_TEST_ARRAY_NO_NAN);
}

#[test]
fn sort_unstable_rev() {
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_unstable_rev();

	let mut rev_sorted_array = SORTED_TEST_ARRAY_NO_NAN;
	rev_sorted_array.reverse();

	assert_eq!(&array[0..N_NO_NAN], &rev_sorted_array);
}

#[test]
fn sort_unstable_by_key() {
	fn key_function(el: &f64) -> f64 {
		(el - 13.0).recip()
	}
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_unstable_by_key(key_function);
	let mut std_sorted_array = TEST_ARRAY_NO_NAN;
	std_sorted_array.sort_unstable_by_key(|num| OrdVar::new(key_function(num)));
	assert_eq!(&array[..N_NO_NAN], &std_sorted_array);
}

// ---------------------------- binary searches --------------------------------

#[test]
fn binary_search() {
	let array = SORTED_TEST_ARRAY;
	for (i, num) in array.iter().enumerate().take(N_NO_NAN) {
		assert_eq!(array.ord_subset_binary_search(&num), Ok(i));
	}
}

#[test]
fn binary_search_rev() {
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_unstable_rev();
	for (i, num) in array.iter().enumerate().take(N_NO_NAN) {
		assert_eq!(array.ord_subset_binary_search_rev(&num), Ok(i));
	}
}

#[test]
fn binary_search_by_key() {
	fn key_function(el: &f64) -> f64 {
		(el - 13.0).recip()
	}
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_unstable_by_key(key_function);
	for num in array.iter().take(N_NO_NAN) {
		let key = key_function(num);
		match array.ord_subset_binary_search_by_key(&key, key_function) {
			Err(_) => panic!("Did not find correct location of element"),
			Ok(pos) => assert_eq!(key_function(&array[pos]), key),
		}
	}
}

// ------ binary search error cases ------

#[test]
fn binary_search_err() {
	let array = SORTED_TEST_ARRAY;
	for (i, num) in array.iter()
		.enumerate()
		.filter(|&(_, num)| num.is_finite())
	{
		let new_num = num + 0.5;
		assert_eq!(array.ord_subset_binary_search(&new_num), Err(i+1));
	}
}

#[test]
fn binary_search_rev_err() {
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_unstable_rev();
	for (i, num) in array.iter()
		.enumerate()
		.filter(|&(_, num)| num.is_finite())
	{
		let new_num = num + 0.5;
		assert_eq!(array.ord_subset_binary_search_rev(&new_num), Err(i));
	}
}

#[test]
fn binary_search_by_key_err() {
	fn key_function(el: &f64) -> f64 {
		(el - 13.0).recip()
	}
	let mut array = TEST_ARRAY;
	array.ord_subset_sort_unstable_by_key(key_function);
	for num in array.iter().take(N_NO_NAN) {
		let key_diff = key_function(&(num+0.01))*1.01 + 0.01;
		let pos = array.ord_subset_binary_search_by_key(&key_diff, key_function);
		let pos_std = (&array[..N_NO_NAN]).binary_search_by_key(
			&OrdVar::new(key_diff),
			|num| OrdVar::new(key_function(num))
		);
		match (pos, pos_std) {
			(Err(pos), Err(pos_std)) => assert!(pos == pos_std),
			// the commented out match branch is also valid behaviour
			// but this function is supposed to test as many error cases as possible
			// by choosing key_diff the right way
			//(Ok(pos), Ok(pos_std)) => {
			//	let key1 = key_function(&array[pos]);
			//	let key2 = key_function(&array[pos_std]);
			//	assert!(key1 == key2);
			//},
			_ => panic!("Inconsistency between this library's and std's binary_search_by_key"),
		}
	}
}

// -------------------- compile time implementation tests ----------------------

// check that slices, arrays and vecs as well as references
// all implement the OrdSubsetSliceExt trait, no matter the mutability.
#[allow(unused)]
fn ord_subset_slice_ext_impl_test() {
	fn foo<T: OrdSubsetSliceExt<U> + AsRef<[U]>, U: OrdSubset + Clone>(as_slice: T) {
		// would panic, good thing it doesn't run
		let element: &U = as_slice.as_ref().first().unwrap();
		as_slice.ord_subset_binary_search(element);
		as_slice.ord_subset_binary_search_rev(element);
		as_slice.ord_subset_binary_search_by_key(element, |_| element.clone());
		as_slice.ord_subset_binary_search_by(|_| std::cmp::Ordering::Equal);
	}
	
	let mut vec: Vec<OrdSub> = vec![];
	let mut arr: [OrdSub; 0] = [];

	// &vec
	foo(&vec);
	foo(&mut vec);

	// &array
	foo(&arr);
	foo(&mut arr);

	// &slice
	foo(&arr[..]);
	foo(&mut arr[..]);

	// &&slice
	foo(&&arr[..]);
	foo(&mut &mut arr[..]);
	foo(& &mut arr[..]);

	// owned
	foo(vec);
	foo(arr);
}

// check that mutable vecs, arrays and slices are all sortable
#[allow(unused)]
fn ord_subset_mut_slice_ext_impl_test() {
	fn sortable<T, U>(mut as_slice: T) 
		where T: OrdSubsetSliceExt<U> + AsMut<[U]>,
		      U: OrdSubset,
	{
		#[cfg(feature="std")]
		as_slice.ord_subset_sort();
		#[cfg(feature="std")]
		as_slice.ord_subset_sort_rev();
		#[cfg(feature="std")]
		as_slice.ord_subset_sort_by(|_, _| core::cmp::Ordering::Equal);
		#[cfg(feature="std")]
		as_slice.ord_subset_sort_by_key(|_| 0.0);

		as_slice.ord_subset_sort_unstable();
		as_slice.ord_subset_sort_unstable_rev();
		as_slice.ord_subset_sort_unstable_by(|_, _| core::cmp::Ordering::Equal);
		as_slice.ord_subset_sort_unstable_by_key(|_| 0.0);
	}

	let mut vec: Vec<OrdSub> = vec![];
	let mut arr: [OrdSub; 0] = [];

	sortable(&mut vec);
	sortable(&mut arr);
	sortable(&mut arr[..]);
	sortable(&mut &mut arr[..]);
	// owned
	sortable(vec);
	sortable(arr);
}

// check that slices, arrays and vecs as well as references of non-OrdSubset items
// all implement the OrdSubsetSliceExt trait and allow binary_search_by_key.
#[allow(unused)]
fn non_ord_subset_slice_ext_impl_test() {
	/*
	fn foo<T: OrdSubsetSliceExt<U> + AsRef<[U]>, U: Clone>(as_slice: T) {
		// would panic, good thing it doesn't run
		let element: &U = as_slice.as_ref().first().unwrap();
		as_slice.ord_subset_binary_search(element);
		as_slice.ord_subset_binary_search_rev(element);
		as_slice.ord_subset_binary_search_by_key(element, |_| element.clone());
		as_slice.ord_subset_binary_search_by(|_| std::cmp::Ordering::Equal);
	}
	*/
	fn foo<T: OrdSubsetSliceExt<U> + AsRef<[U]>, U>(as_slice: T) {
		let key = OrdSub();
		as_slice.ord_subset_binary_search_by_key(&key, |_| key);
	}

	let mut vec: Vec<NotOrdSub> = vec![];
	let mut arr: [NotOrdSub; 0] = [];

	// &vec
	foo(&vec);
	foo(&mut vec);

	// &array
	foo(&arr);
	foo(&mut arr);

	// &slice
	foo(&arr[..]);
	foo(&mut arr[..]);

	// &&slice
	foo(&&arr[..]);
	foo(&mut &mut arr[..]);
	foo(& &mut arr[..]);

	// owned
	foo(vec);
	foo(arr);
}

// check that mutable vecs, arrays and slices of non-OrdSubset types are all sortable by key
#[allow(unused)]
fn non_ord_subset_mut_slice_ext_impl_test() {
	fn sortable<T, U>(mut as_slice: T) 
		where T: OrdSubsetSliceExt<U> + AsMut<[U]>,
	{
		let key = OrdSub();

		#[cfg(feature="std")]
		as_slice.ord_subset_sort_by_key(|_| key);

		as_slice.ord_subset_sort_unstable_by_key(|_| key);
	}

	let mut vec: Vec<NotOrdSub> = vec![];
	let mut arr: [NotOrdSub; 0] = [];

	sortable(&mut vec);
	sortable(&mut arr);
	sortable(&mut arr[..]);
	sortable(&mut &mut arr[..]);
	// owned
	sortable(vec);
	sortable(arr);
}

// std-library bug: https://github.com/rust-lang/rust/issues/34683
// caused valid code not to compile due to elided lifetime parameters being too strict
// this test is a compile test, it can't fail at runtime
#[test]
fn binary_search_lifetime() {
	#[derive(Debug)]
	struct Foo {
    	property: f32,
	}

    let xs = vec![
        Foo { property: 1. },
        Foo { property: 2. },
        Foo { property: 3. },
    ];

    let _r = xs.ord_subset_binary_search_by_key(&2., |entry| entry.property);
}

#[cfg(ops)]
fn ops_impl_test() {
	let mut float = OrdVar::new(1.0);
	float += float;
	float -= float;
	float *= float;
	float /= float;
	float %= float;
}