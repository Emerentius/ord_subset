// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ord_subset_trait::*;
use core::cmp::Ordering::{self, Greater, Equal, Less};

// Wrapper for comparison functions
// Treats unordered values as greater than any ordered
fn compare_unordered_greater_everything<T: OrdSubset, F>(a: &T, b: &T, mut compare: F) -> Ordering
	where F: FnMut(&T, &T) -> Ordering,
{
	match (a.is_outside_order(), b.is_outside_order()) {
			// catch invalids and put them at the end
			// Ordering of two-non-ords in the (true, true) case is irrelevant
			// for the goal of collecting them at the end. However, comparing them
			// as equal will let the algorithm uphold its stability properties
			(true, true) => Equal,
			(true, false) => Greater,
			(false, true) => Less,
			(false, false) => compare(a,b), // the normal case, both valid. Here user function applies.
	}
}

pub trait OrdSubsetSliceExt<T> {
	/// Sort the slice. Values outside the ordered subset are put at the end in their original order.
	///
	/// This is equivalent to `self.ord_subset_sort_by(|a,b| a.partial_cmp(b).unwrap())`
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	#[cfg(feature="std")]
	fn ord_subset_sort(&mut self) where T: OrdSubset;

	/// **UNSTABLE** Will likely remove these. Easily recreated by `.sort_by()`
	///
	/// Sort the slice in reverse order. Values outside the ordered subset are put at the end in their original order (i.e. not reversed).
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	#[cfg(feature="std")]
	fn ord_subset_sort_rev(&mut self) where T: OrdSubset;

	/// Sorts the slice, using `compare` to order elements. Values outside the total order are put at the end in their original order.
	/// `compare` will not be called on them. If you wish to handle these yourself, use the regular `.sort_by()`.
	///
	/// **Warning:** The function interface is identical to the `.sort_by()` interface. Be careful not to miss `ord_subset_` in front.
	/// It would work until you have unordered values in your slice, then crash unexpectedly.
	///
	/// This delegates to `.sort_by()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) for
	/// time and space complexity of the current implementation.
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	#[cfg(feature="std")]
	fn ord_subset_sort_by<F>(&mut self, compare: F)
		where T: OrdSubset,
		      F: FnMut(&T, &T) -> Ordering;

	/// Sorts the slice, using `key` to extract a key by which to order the sort by. Entries mapping to values outside
	/// the total order will be put at the end in their original order.
	///
	/// This delegates to `.sort_by()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) for
	/// time and space complexity of the current implementation.
	#[cfg(feature="std")]
	fn ord_subset_sort_by_key<B, F>(&mut self, f: F)
		where B: OrdSubset,
		      F: FnMut(&T) -> B;

	/// Sort the slice. Values outside the ordered subset are put at the end.
	///
	/// This is equivalent to `self.ord_subset_sort_by(|a,b| a.partial_cmp(b).unwrap())`
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	fn ord_subset_sort_unstable(&mut self) where T: OrdSubset;
	
	/// **UNSTABLE** Will likely remove these. Easily recreated by `.sort_by()`
	///
	/// Sort the slice in reverse order. Values outside the ordered subset are put at the end.
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	fn ord_subset_sort_unstable_rev(&mut self) where T: OrdSubset;

	/// Sorts the slice, using `compare` to order elements. Values outside the total order are put at the end.
	/// `compare` will not be called on them. If you wish to handle these yourself, use the regular `.sort_unstable_by()`.
	///
	/// **Warning:** The function interface is identical to the `.sort_unstable_by()` interface. Be careful not to miss `ord_subset_` in front.
	/// It would work until you have unordered values in your slice, then crash unexpectedly.
	///
	/// This delegates to `.sort_by_unstable()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_unstable) for
	/// time and space complexity of the current implementation.
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	fn ord_subset_sort_unstable_by<F>(&mut self, compare: F)
		where T: OrdSubset,
		      F: FnMut(&T, &T) -> Ordering;

	/// Sorts the slice, using `key` to extract a key by which to order the sort by. Entries mapping to values outside
	/// the total order will be put at the end.
	///
	/// This delegates to `.sort_by_unstable()` in the std library. See [official docs](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_unstable) for
	/// time and space complexity of the current implementation.
	fn ord_subset_sort_unstable_by_key<B, F>(&mut self, f: F)
		where B: OrdSubset,
		      F: FnMut(&T) -> B;

	/// Binary search a sorted slice for a given element. Values outside the ordered subset need to be at the end of the slice.
	///
	/// If the value is found then Ok is returned, containing the index of the matching element; if the value is not found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	///
	/// # Example
	///
	/// Looks up a series of five elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1,4]`.
	///
	/// ```
	/// use ord_subset::OrdSubsetSliceExt;
	/// use std::f64;
	///
	/// let s = [0., 1., 1., 1., 1., 2., 3., 5., 8., 13., 21., 34., 55., f64::NAN, f64::NAN];
	///
	/// assert_eq!(s.ord_subset_binary_search(&13.),  Ok(9));
	/// assert_eq!(s.ord_subset_binary_search(&4.),   Err(7));
	/// assert_eq!(s.ord_subset_binary_search(&100.), Err(13));
	/// let r = s.ord_subset_binary_search(&1.);
	/// assert!(match r { Ok(1...4) => true, _ => false, });
	///	assert_eq!(s.ord_subset_binary_search(&f64::INFINITY), Err(13));
	/// ```
	///
	/// # Panics
	///
	/// Panics if the argument is outside of the total order. Also panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	fn ord_subset_binary_search(&self, x: &T) -> Result<usize, usize> where T: OrdSubset;

	/// Binary search a sorted slice with a comparator function.
	///
	/// The comparator function should implement an order consistent with the sort order of the underlying slice, returning an order code that indicates whether its argument is Less, Equal or Greater the desired target. The comparator will only be called for values inside the total order.
	///
	/// It's imperative, that the comparator function doesn't compare its arguments with values outside the total order. This will result in bogus output which cannot be caught by this function.
	///
	/// If a matching value is found then returns Ok, containing the index for the matched element; if no match is found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	fn ord_subset_binary_search_by<F>(&self, f: F) -> Result<usize, usize>
		where T: OrdSubset,
		      F: FnMut(&T) -> Ordering;

	/// Binary search a sorted slice with a key extraction function.
	///
	/// Assumes that the slice is sorted by the key, for instance with `ord_subset_sort_by_key` using the same key extraction function.
	///
	/// If a matching value is found then returns `Ok`, containing the index for the matched element; if no match is found then `Err` is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	fn ord_subset_binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
		where B: OrdSubset,
		      F: FnMut(&T) -> B;

	/// **UNSTABLE** Will likely remove these. Easily recreated by `.binary_search_by()`
	///
	/// Binary search a slice sorted in reverse order for a given element. Values outside the ordered subset need to be at the end of the slice.
	///
	/// If a matching value is found then returns Ok, containing the index for the matched element; if no match is found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	///
	/// # Panics
	///
	/// Panics if the argument is outside of the total order. Also panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated OrdSubset contract).
	fn ord_subset_binary_search_rev(&self, x: &T) -> Result<usize, usize> where T: OrdSubset;
}

impl<T> OrdSubsetSliceExt<T> for [T]
{
	#[cfg(feature="std")]
	fn ord_subset_sort(&mut self)
		where T: OrdSubset,
	{
		self.ord_subset_sort_by(|a,b| a.partial_cmp(b).expect("Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order"))
	}

	#[cfg(feature="std")]
	fn ord_subset_sort_by<F>(&mut self, mut compare: F)
		where T: OrdSubset,
		      F: FnMut(&T, &T) -> Ordering
	{
		self.sort_by(|a, b|
			compare_unordered_greater_everything(a, b, &mut compare)
		)
	}

	#[cfg(feature="std")]
	fn ord_subset_sort_rev(&mut self)
		where T: OrdSubset,
	{
		self.ord_subset_sort_by(|a,b| b.partial_cmp(a).expect("Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order"))
	}

	#[cfg(feature="std")]
	fn ord_subset_sort_by_key<B, F>(&mut self, mut f: F)
		where B: OrdSubset,
		      F: FnMut(&T) -> B
	{
		// FIXME: This is a contract error, not a library error (error message)
		let cmp_ord = |a: &B, b: &B| a.partial_cmp(b).expect("Internal ord_subset error. Reached supposedly unreachable code path in ord_subset_binary_search_by_key");

		self.sort_by(|a, b| compare_unordered_greater_everything(&(f(a)), &(f(b)), &cmp_ord))
	}

	fn ord_subset_sort_unstable(&mut self)
		where T: OrdSubset,
	{
		self.ord_subset_sort_unstable_by(|a,b| a.partial_cmp(b).expect("Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order"))
	}

	fn ord_subset_sort_unstable_by<F>(&mut self, mut compare: F)
		where T: OrdSubset,
		      F: FnMut(&T, &T) -> Ordering
	{
		self.sort_unstable_by(|a, b|
			compare_unordered_greater_everything(a, b, &mut compare)
		)
	}
	fn ord_subset_sort_unstable_rev(&mut self)
		where T: OrdSubset,
	{
		self.ord_subset_sort_unstable_by(|a,b| b.partial_cmp(a).expect("Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order"))
	}

	fn ord_subset_sort_unstable_by_key<B, F>(&mut self, mut f: F)
		where B: OrdSubset,
		      F: FnMut(&T) -> B
	{
		// FIXME: This is a contract error, not a library error (error message)
		let cmp_ord = |a: &B, b: &B| a.partial_cmp(b).expect("Internal ord_subset error. Reached supposedly unreachable code path in ord_subset_binary_search_by_key");

		self.sort_unstable_by(|a, b| compare_unordered_greater_everything(&(f(a)), &(f(b)), &cmp_ord))
	}

	fn ord_subset_binary_search(&self, x: &T) -> Result<usize, usize>
		where T: OrdSubset,
	{
		if x.is_outside_order() { panic!("Attempted binary search for value outside total order") };
		self.ord_subset_binary_search_by(|other| {
			other.partial_cmp(x).expect("Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order")
		})
	}

	fn ord_subset_binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
		where T: OrdSubset,
		      F: FnMut(&T) -> Ordering
	{
		self.binary_search_by( |other| {
			match other.is_outside_order() {
				true  => Greater, // unordered always at end
				false => f(other),
			}
		})
	}

	fn ord_subset_binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
		where B: OrdSubset,
		      F: FnMut(&T) -> B
	{
		if b.is_outside_order() { panic!("Attempted binary search for value outside total order") };
		// compare ordered values as expected
		// wrap it in a function that deals with unordered, so this one never sees them
		let cmp_ord = |a: &B, b: &B| a.partial_cmp(b).expect("Internal ord_subset error. Reached supposedly unreachable code path in ord_subset_binary_search_by_key");
		self.binary_search_by(|k| compare_unordered_greater_everything(&f(k), b, &cmp_ord))
	}

	fn ord_subset_binary_search_rev(&self, x: &T) -> Result<usize, usize>
		where T: OrdSubset,
	{
		if x.is_outside_order() { panic!("Attempted binary search for value outside total order") };
		self.ord_subset_binary_search_by(|other| {
			x.partial_cmp(other).expect("Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order")
		})
	}
}

impl<T, U> OrdSubsetSliceExt<T> for U
	where U: AsRef<[T]> + AsMut<[T]>,
	      [T]: OrdSubsetSliceExt<T>,
{
	#[cfg(feature="std")]
	fn ord_subset_sort(&mut self)
		where T: OrdSubset,
	{
		self.as_mut().ord_subset_sort()
	}

	#[cfg(feature="std")]
	fn ord_subset_sort_by<F>(&mut self, compare: F)
		where T: OrdSubset,
		      F: FnMut(&T, &T) -> Ordering,
	{
		self.as_mut().ord_subset_sort_by(compare)
	}

	#[cfg(feature="std")]
	fn ord_subset_sort_rev(&mut self)
		where T: OrdSubset,
	{
		self.as_mut().ord_subset_sort_rev();
	}

	#[cfg(feature="std")]
	fn ord_subset_sort_by_key<B, F>(&mut self, f: F)
		where B: OrdSubset,
		      F: FnMut(&T) -> B
	{
		self.as_mut().ord_subset_sort_by_key(f)
	}

	fn ord_subset_sort_unstable(&mut self)
		where T: OrdSubset,
	{
		self.as_mut().ord_subset_sort_unstable()
	}

	fn ord_subset_sort_unstable_by<F>(&mut self, compare: F)
		where T: OrdSubset,
		      F: FnMut(&T, &T) -> Ordering,
	{
		self.as_mut().ord_subset_sort_unstable_by(compare)
	}

	fn ord_subset_sort_unstable_rev(&mut self)
		where T: OrdSubset,
	{
		self.as_mut().ord_subset_sort_unstable_rev();
	}

	fn ord_subset_sort_unstable_by_key<B, F>(&mut self, f: F)
		where B: OrdSubset,
		      F: FnMut(&T) -> B
	{
		self.as_mut().ord_subset_sort_unstable_by_key(f)
	}

	fn ord_subset_binary_search(&self, x: &T) -> Result<usize, usize>
		where T: OrdSubset,
	{
		self.as_ref().ord_subset_binary_search(x)
	}

	fn ord_subset_binary_search_by<F>(&self, f: F) -> Result<usize, usize>
		where //U: AsRef<[T]>,
		      T: OrdSubset,
		      F: FnMut(&T) -> Ordering
	{
		self.as_ref().ord_subset_binary_search_by(f)
	}

	fn ord_subset_binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
		where B: OrdSubset,
		      F: FnMut(&T) -> B
	{
		self.as_ref().ord_subset_binary_search_by_key(b, f)
	}

	fn ord_subset_binary_search_rev(&self, x: &T) -> Result<usize, usize>
		where // U: AsRef<[T]>,
		      T: OrdSubset,
	{
		self.as_ref().ord_subset_binary_search_rev(x)
	}
}
