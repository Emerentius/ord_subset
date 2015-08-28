// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ord_subset_trait::*;
use ord_var::*;
// for min_by
//use rev_option::*;
// min_max() feature currently not used
//use std::iter::MinMaxResult;

/////////////////////////////////////////////////////////////////////
pub trait OrdSubsetIterExt: Iterator
	where Self::Item: OrdSubset
{
	/// Consumes the entire iterator to return the maximum element.
	/// Values outside the ordered subset as given by `.is_outside_order()` are ignored.
	///
	/// Returns the leftmost element if the comparison determines two elements to be equally maximum.
	///
	/// # Example
	///
	/// ```
	/// use ord_subset::OrdSubsetIterExt;
	///
	/// let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	/// let max = vec.iter().ord_subset_max().unwrap();
	/// assert_eq!(&5.0, max);
	/// ```
	fn ord_subset_max(self) -> Option<Self::Item>;


	/// Consumes the entire iterator to return the minimum element.
	/// Values outside the ordered subset as given by `.is_outside_order()` are ignored.
	///
	/// Returns the leftmost element if the comparison determines two elements to be equally minimum.
	///
	/// # Example
	///
	/// ```
	/// use ord_subset::OrdSubsetIterExt;
	///
	/// let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	/// let min = vec.iter().ord_subset_min().unwrap();
	/// assert_eq!(&2.0, min);
	/// ```
	fn ord_subset_min(self) -> Option<Self::Item>;
/*
	/// **UNSTABLE** Follows the std library.
	///
	/// `min_max` finds the minimum and maximum elements in the iterator.
	///
	/// The return type `MinMaxResult` is an enum of three variants:
	///
	/// * `NoElements` if the iterator is empty.
    /// * `OneElement(x)` if the iterator has exactly one element.
    /// * `MinMax(x, y)` is returned otherwise, where `x <= y`. Two values are equal if and only if there is more than one element in the iterator and all elements are equal.
	///
	/// On an iterator of length `n`, `min_max` does `1.5 * n` comparisons, and so is faster than calling `min` and `max` separately which does `2 * n` comparisons.
	fn ord_subset_min_max(self) -> MinMaxResult<Self::Item>;

	/// **UNSTABLE** Follows the std library.
	///
	/// Returns the element that gives the minimum value from the specified function.
	/// Values outside the ordered subset as given by `.is_outside_order()` on the mapped value are ignored.
	///
	/// Returns the rightmost element if the comparison determines two elements to be equally minimum.
	///
	/// # Example
	///
	/// ```
	/// extern crate ord_subset;
	/// use ord_subset::OrdSubsetIterExt;
	///
	/// fn main() {
	/// 	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	/// 	let min_by = vec.iter().ord_subset_min_by(|num| num.recip()).unwrap();
	/// 	assert_eq!(&5.0, min_by);
	/// }
	/// ```
	fn ord_subset_min_by<F, B>(self, f: F) -> Option<Self::Item>
		where F: FnMut(&Self::Item) -> B,
			  B: OrdSubset;

	/// **UNSTABLE** Follows the std library.
	///
	/// Returns the element that gives the maximum value from the specified function.
	/// Values outside the ordered subset as given by `.is_outside_order()` on the mapped value are ignored.
	///
	/// Returns the rightmost element if the comparison determines two elements to be equally maximum.
	fn ord_subset_max_by<F, B>(self, f: F) -> Option<Self::Item>
		where F: FnMut(&Self::Item) -> B,
			  B: OrdSubset;
*/
}

impl<T: Iterator> OrdSubsetIterExt for T
	where T::Item: OrdSubset
{
	fn ord_subset_max(self) -> Option<Self::Item> {
		self.filter_map(OrdVar::new_checked)
			.max()
			.map(|m| m.into_inner()) // Option<OrdVar<Item>> => Option<Item>
	}

	fn ord_subset_min(self) -> Option<Self::Item> {
		self.filter_map(OrdVar::new_checked)
			.min()
			.map(|m| m.into_inner()) // Option<OrdVar<Item>> => Option<Item>
	}
/*
	fn ord_subset_min_max(self) -> MinMaxResult<Self::Item> {
		use std::iter::MinMaxResult::*;
		match self.filter_map(OrdVar::new_checked)
			.min_max()
		{
			NoElements    => NoElements,
			OneElement(a) => OneElement(a.into_inner()),
			MinMax(a,b)   => MinMax(a.into_inner(), b.into_inner()),
		}
	}

	fn ord_subset_min_by<F, B>(self, mut f: F) -> Option<Self::Item>
		where F: FnMut(&Self::Item) -> B,
			  B: OrdSubset
	{
		// None < Some, always
		self.min_by(|it| RevOption(OrdVar::new_checked(f(it))))
	}

	fn ord_subset_max_by<F, B>(self, mut f: F) -> Option<Self::Item>
		where F: FnMut(&Self::Item) -> B,
		      B: OrdSubset,
	{
		// Some > None, always
		self.max_by(|it| OrdVar::new_checked(f(it)))
	}
*/
}
