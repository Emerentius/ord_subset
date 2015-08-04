use almost_ord_trait::*;

pub trait AlmostOrdSliceExt<T: AlmostOrd> {
	/// Sort the slice, in place. Values outside the ordered subset are put at the end in no particular order.
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated AlmostOrd contract).
	fn partial_sort(&mut self);

	/// Sort the slice in reverse order, in place. Values outside the ordered subset are put at the end in no particular order.
	///
	/// # Panics
	///
	/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated AlmostOrd contract).
	fn partial_sort_rev(&mut self);

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
		self.sort_by(|a,b| {
			match a.partial_cmp(b) {
				Some(ord) => ord,
				None      => {
					use std::cmp::Ordering::*;
					match (a.is_outside_order(), b.is_outside_order()) {
						(true, false) | (true, true) => Greater, // (true, true) Ordering is irrelevant
						(false, true) => Less,
						(false, false) => unreachable!(), // or illegal implementation of AlmostOrd
					}
				}
			}
		})
	}

	fn partial_sort_rev(&mut self) {
		self.sort_by(|a,b| {
			match b.partial_cmp(a) { // <-- reverse
				Some(ord) => ord,
				None      => {
					use std::cmp::Ordering::*;
					match (b.is_outside_order(), a.is_outside_order()) { // <-- reverse
						// true, false & false, true reversed so invalids still land at end
						// two negatives => positive
						(true, false) | (true, true) => Less, // (true, true) Ordering is irrelevant
						(false, true) => Greater,
						(false, false) => unreachable!(), // or illegal implementation of AlmostOrd
					}
				}
			}
		})
	}

	fn partial_binary_search(&self, x: &T) -> Result<usize, usize> {
		if x.is_outside_order() { panic!("Attempted binary search for value outside total order") };
		self.binary_search_by( |other| {
			use std::cmp::Ordering::*;
			match other.is_outside_order() {
				true  => Greater,
				false => other.partial_cmp(x).expect("Violated AlmostOrd contract: a.partial_cmp(b) == None for a,b inside total order")
			}
		})
	}


	fn partial_binary_search_rev(&self, x: &T) -> Result<usize, usize> {
		if x.is_outside_order() { panic!("Attempted binary search for value outside total order") };
		self.binary_search_by( |other| {
			use std::cmp::Ordering::*;
			match other.is_outside_order() {
				true  => Greater, // <-- same because invalid values always at end
				// x and other reverse
				false => x.partial_cmp(other).expect("Violated AlmostOrd contract: a.partial_cmp(b) == None for a,b inside total order")
			}
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

	fn partial_sort_rev(&mut self) {
		self.as_mut().partial_sort_rev();
	}

	fn partial_binary_search(&self, x: &T) -> Result<usize, usize> {
		self.as_ref().partial_binary_search(x)
	}

	fn partial_binary_search_rev(&self, x: &T) -> Result<usize, usize> {
		self.as_ref().partial_binary_search_rev(x)
	}
}
