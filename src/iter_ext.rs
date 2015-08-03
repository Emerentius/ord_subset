use almost_ord_trait::*;
use ord_val::*;
use rev_option::*;

/////////////////////////////////////////////////////////////////////
pub trait AlmostOrdIterExt<T>: Iterator
	where <T as Iterator>::Item: AlmostOrd
{
	/// Consumes the entire iterator to return the maximum element.
	/// Values outside the ordered subset as given by ```.is_outside_order()``` are ignored.
	///
	/// Returns the leftmost element if the comparison determines two elements to be equally maximum.
	///
	/// # Example
	///
	/// ```
	/// extern crate almost_ord;
	/// use almost_ord::AlmostOrdIterExt;
	///
	/// fn main() {
	/// 	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	/// 	let max = vec.iter().partial_max().unwrap();
	/// 	assert_eq!(&5.0, max);
	///	}
	/// ```
	fn partial_max(self) -> Option<<T as Iterator>::Item>;


	/// Consumes the entire iterator to return the minimum element.
	/// Values outside the ordered subset as given by ```.is_outside_order()``` are ignored.
	///
	/// Returns the leftmost element if the comparison determines two elements to be equally minimum.
	///
	/// # Example
	///
	/// ```
	/// extern crate almost_ord;
	/// use almost_ord::AlmostOrdIterExt;
	///
	/// fn main() {
	/// 	let vec = vec![2.0, 3.0, 5.0, std::f64::NAN];
	/// 	let min = vec.iter().partial_min().unwrap();
	/// 	assert_eq!(&2.0, min);
	///	}
	/// ```
	fn partial_min(self) -> Option<<T as Iterator>::Item>;
	//fn partial_min_max(self) -> Option<<T as Iterator>::Item>;

	/// Follows the std library. Subject to change as min_by is unstable.
	///
	/// Returns the element that gives the minimum value from the specified function.
	/// Values outside the ordered subset as given by ```.is_outside_order()``` on the mapped value are ignored.
	///
	/// Returns the rightmost element if the comparison determines two elements to be equally minimum.
	fn partial_min_by<F, B>(self, f: F) -> Option<<T as Iterator>::Item>
		where F: FnMut(&<T as Iterator>::Item) -> B,
			  B: AlmostOrd;

	/// Follows the std library. Subject to change as max_by is unstable.
	///
	/// Returns the element that gives the maximum value from the specified function.
	/// Values outside the ordered subset as given by ```.is_outside_order()``` on the mapped value are ignored.
	///
	/// Returns the rightmost element if the comparison determines two elements to be equally maximum.
	fn partial_max_by<F, B>(self, f: F) -> Option<<T as Iterator>::Item>
		where F: FnMut(&<T as Iterator>::Item) -> B,
			  B: AlmostOrd;
}

impl<T> AlmostOrdIterExt<T> for T
	where T: Iterator,
	      <T as Iterator>::Item: AlmostOrd
{
	fn partial_max(self) -> Option<Self::Item> {
		self.filter(AlmostOrd::is_on_order)
			.map(OrderedVal::new_unchecked)
			.max()
			.map(|m| m.0) // Option<OrderedVal<Item>> => Option<Item>
	}

	fn partial_min(self) -> Option<Self::Item> {
		self.filter(AlmostOrd::is_on_order)
			.map(OrderedVal::new_unchecked)
			.min()
			.map(|m| m.0) // Option<OrderedVal<Item>> => Option<Item>
	}

	fn partial_min_by<F, B>(self, mut f: F) -> Option<Self::Item>
		where F: FnMut(&<T as Iterator>::Item) -> B,
			  B: AlmostOrd
	{
		// None > Some, always
		self.min_by(|it| RevOption(OrderedVal::new_checked(f(it))))
	}

	fn partial_max_by<F, B>(self, mut f: F) -> Option<Self::Item>
		where F: FnMut(&Self::Item) -> B,
		      B: AlmostOrd,
	{
		// Some > None, always
		self.max_by(|it| OrderedVal::new_checked(f(it)))
	}
}
