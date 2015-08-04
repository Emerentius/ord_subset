use almost_ord_trait::*;
use std::cmp::Ordering::{self, Greater, Less};

pub trait AlmostOrdSliceExt<T: AlmostOrd> {
	/// Sort the slice, in place. Values outside the ordered subset are put at the end in no particular order.
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated AlmostOrd contract).
	fn partial_sort(&mut self);


	/// **UNSTABLE** Will likely remove these. Too specific and easily recreated by `.sort_by()`
	///
	/// Sort the slice in reverse order, in place. Values outside the ordered subset are put at the end in no particular order.
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated AlmostOrd contract).
	fn partial_sort_rev(&mut self);

	/// **Warning:** The function interface is equal to the `.sort_by()` interface. Be careful not to miss a `partial_` in front. It would work until you have unordered values in your slice, then crash unexpectedly.
	///
	/// Sorts the slice, in place, using compare to compare elements. Values outside the total order are put at the end. The comparator will not be called on them. If you wish to handle these yourself, use the regular `.sort_by()`.
	/// The argument `compare` will only be used to compare elementns inside the total order.
	///
	/// This sort is O(n log n) worst-case and stable, but allocates approximately 2 * n, where n is the length of self.
	///
	/// # Panics
	///
	/// This method doesn't panic on its own. However, if `AlmostOrd` was implemented incorrectly, `unwrap`ping the result of `a.partial_cmp(b)` inside `compare` could panic.
	/// Apart from that possibility, unwrapping is safe in that situation.
	fn partial_sort_by<F>(&mut self, compare: F)
		where F: FnMut(&T, &T) -> Ordering;

	/// Binary search a sorted slice for a given element. Values outside the ordered subset need to be at the end of the slice.
	///
	/// If the value is found then Ok is returned, containing the index of the matching element; if the value is not found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	///
	/// # Example
	///
	/// Looks up a series of five elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in ``[1,4].
	///
	/// ```
	/// extern crate almost_ord;
	/// use almost_ord::AlmostOrdSliceExt;
	/// use std::f64;
	///
	/// fn main() {
	/// 	let s = [0., 1., 1., 1., 1., 2., 3., 5., 8., 13., 21., 34., 55., f64::NAN, f64::NAN];
	///
	/// 	assert_eq!(s.partial_binary_search(&13.),  Ok(9));
	/// 	assert_eq!(s.partial_binary_search(&4.),   Err(7));
	/// 	assert_eq!(s.partial_binary_search(&100.), Err(13));
	/// 	let r = s.partial_binary_search(&1.);
	/// 	assert!(match r { Ok(1...4) => true, _ => false, });
	///		assert_eq!(s.partial_binary_search(&f64::INFINITY), Err(13));
	/// }
	/// ```
	///
	/// # Panics
	///
	/// Panics if the argument is outside of the total order. Also panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated AlmostOrd contract).
	fn partial_binary_search(&self, x: &T) -> Result<usize, usize>;

	/// Binary search a sorted slice with a comparator function.
	///
	/// The comparator function should implement an order consistent with the sort order of the underlying slice, returning an order code that indicates whether its argument is Less, Equal or Greater the desired target. The comparator will only be called for values inside the total order.
	///
	/// **It's imperative, that the comparator function doesn't compare with values outside the total order. This will lead to logic errors which cannot be caught by this function.** You can use `AlmostOrd::is_outside_order(elem)` inside the comparator to distinguish.
	///
	/// If a matching value is found then returns Ok, containing the index for the matched element; if no match is found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	fn partial_binary_search_by<F>(&self, f: F) -> Result<usize, usize>
		where F: FnMut(&T) -> Ordering;

	/// **UNSTABLE** Will likely remove these. Too specific and easily recreated by `.binary_search_by()`
	///
	/// Binary search a slice sorted in reverse order for a given element. Values outside the ordered subset need to be at the end of the slice.
	///
	/// If the value is found then Ok is returned, containing the index of the matching element; if the value is not found then Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
	///
	/// # Panics
	///
	/// Panics if the argument is outside of the total order. Also panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated AlmostOrd contract).
	fn partial_binary_search_rev(&self, x: &T) -> Result<usize, usize>;
}

impl<T> AlmostOrdSliceExt<T> for [T]
	where T: AlmostOrd
{
	fn partial_sort(&mut self) {
		self.partial_sort_by(|a,b| a.partial_cmp(b).expect("Violated AlmostOrd contract: a.partial_cmp(b) == None for a,b inside total order"))
	}

	fn partial_sort_by<F>(&mut self, mut compare: F)
		where F: FnMut(&T, &T) -> Ordering
	{
		self.sort_by(|a,b| {
			match (a.is_outside_order(), b.is_outside_order()) {
				// catch invalids and put them at the end
				(true, false) | (true, true) => Greater, // (true, true) Ordering is irrelevant
				(false, true) => Less,
				(false, false) => compare(a,b), // the normal case, both valid. Here user function applies.
			}
		})
	}

	fn partial_sort_rev(&mut self) {
		self.partial_sort_by(|a,b| b.partial_cmp(a).expect("Violated AlmostOrd contract: a.partial_cmp(b) == None for a,b inside total order"))
	}

	fn partial_binary_search(&self, x: &T) -> Result<usize, usize> {
		if x.is_outside_order() { panic!("Attempted binary search for value outside total order") };
		self.partial_binary_search_by(|other| {
			other.partial_cmp(x).expect("Violated AlmostOrd contract: a.partial_cmp(b) == None for a,b inside total order")
		})
	}

	fn partial_binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
		where F: FnMut(&T) -> Ordering
	{
		self.binary_search_by( |other| {
			match other.is_outside_order() {
				true  => Greater, // unordered always at end
				false => f(other),
			}
		})
	}

	fn partial_binary_search_rev(&self, x: &T) -> Result<usize, usize> {
		if x.is_outside_order() { panic!("Attempted binary search for value outside total order") };
		self.partial_binary_search_by(|other| {
			x.partial_cmp(other).expect("Violated AlmostOrd contract: a.partial_cmp(b) == None for a,b inside total order")
		})
	}
}

impl<T, U> AlmostOrdSliceExt<T> for U
	where T: AlmostOrd,
		  U: AsMut<[T]> + AsRef<[T]>,
{
	fn partial_sort(&mut self) {
		self.as_mut().partial_sort();
	}

	fn partial_sort_by<F>(&mut self, compare: F)
		where F: FnMut(&T, &T) -> Ordering
	{
		self.as_mut().partial_sort_by(compare);
	}

	fn partial_sort_rev(&mut self) {
		self.as_mut().partial_sort_rev();
	}

	fn partial_binary_search(&self, x: &T) -> Result<usize, usize> {
		self.as_ref().partial_binary_search(x)
	}

	fn partial_binary_search_by<F>(&self, f: F) -> Result<usize, usize>
		where F: FnMut(&T) -> Ordering
	{
		self.as_ref().partial_binary_search_by(f)
	}

	fn partial_binary_search_rev(&self, x: &T) -> Result<usize, usize> {
		self.as_ref().partial_binary_search_rev(x)
	}
}
