#![feature(iter_cmp)]
use std::convert::AsRef;
use std::cmp::{Ord, Eq, Ordering};
use std::fmt::Debug;

/// This trait is for types that have a total ordering except for some elements which fall outside the line.
/// ```is_outside_order()``` must return true for these outliers and false for anything else.
///
/// The method ```partial_cmp(a,b)``` must return ```Some(_)``` if ```is_outside_order(a)``` == ```is_outside_order(b)``` == ```false``` and ```None``` else.
pub trait AlmostOrd: PartialOrd<Self> + PartialEq<Self> {
	fn is_outside_order(&self) -> bool;

	#[inline(always)]
	fn is_on_order(&self) -> bool {
		!self.is_outside_order()
	}
}

impl<'a, A> AlmostOrd for &'a A where A: AlmostOrd {
	fn is_outside_order(&self) -> bool {
		(*self).is_outside_order()
	}
}

impl<'a, A> AlmostOrd for &'a mut A where A: AlmostOrd {
	fn is_outside_order(&self) -> bool {
		(**self).is_outside_order()
	}
}

// helper struct for AlmostOrd values. It's illegal to save values outside order in this.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct OrderedVal<T: AlmostOrd>(T);

#[allow(dead_code)]
impl<T: AlmostOrd> OrderedVal<T> {
	fn new(data: T)	-> OrderedVal<T>
		where T: Debug
	{
		if data.is_outside_order() { panic!("Tried to save unordered data into OrderedVal: {:?}", data) };
		OrderedVal(data)
	}

	fn new_checked(data: T)	-> Option<OrderedVal<T>>
	{
		match data.is_outside_order() {
			true  => None,
			false => Some(OrderedVal(data)),
		}
	}

	fn new_unchecked(data: T) -> OrderedVal<T> {
		OrderedVal(data)
	}
}

impl<T: AlmostOrd> Eq for OrderedVal<T> {}

impl<T: AlmostOrd> Ord for OrderedVal<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).expect("Violated AlmostOrd contract. OrderedVal internal wrapper contains non-ordered value")
	}
}
///////////////////////////////////////////
#[derive(Debug, PartialEq, Eq)]
struct RevOption<T>(Option<T>);

impl<T: PartialOrd> PartialOrd<RevOption<T>> for RevOption<T> {
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		other.0.partial_cmp(&self.0) // reverse!
	}
}

impl<T: Ord> Ord for RevOption<T> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		other.0.cmp(&self.0) // reverse!
	}
}

///////////////////////////////////////////
impl AlmostOrd for f64 {
	fn is_outside_order(&self) -> bool {
		self.is_nan()
	}
}

impl AlmostOrd for f32 {
	fn is_outside_order(&self) -> bool {
		self.is_nan()
	}
}

pub trait AlmostOrdSliceExt<T: AlmostOrd> {
	/// Sort the slice, in place. Values outside the ordered subset are put at the end in no particular order.
	fn partial_sort(&mut self);
	fn partial_binary_search(&self, x: &T) -> Result<usize, usize>;
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
						(true, false) => Greater,
						(false, true) => Less,
						(true, true)  => Greater, // or whatever
						(false, false) => unreachable!(),
					}
				}
			}
		})
	}

	fn partial_binary_search(&self, x: &T) -> Result<usize, usize> {
		if x.is_outside_order() { panic!("Attempted binary search for value v, v.is_outside_order == true") };
		self.binary_search_by( |other| {
			use std::cmp::Ordering::*;
			match other.is_outside_order() {
				true  => Greater,
				false => other.partial_cmp(x).expect("Violated AlmostOrd contract: .partial_cmp() returned 'None' on comparison of two values for which .is_outside_order() is false")
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

	fn partial_binary_search(&self, x: &T) -> Result<usize, usize> {
		self.as_ref().partial_binary_search(x)
	}
}
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
		// None < Some, always
		self.min_by(|it| RevOption(OrderedVal::new_checked(f(it))))
		/*
		let mut iter = self.map(|it| (f(&it), it))
			.filter(|tup| !tup.0.is_outside_order());

		// find first valid element, because None < Some(_) == true, always
		let (mut min_mapped, mut min) = match iter.next() {
			Some((it_m, it)) => (Some(it_m), Some(it)),
			None		     => return None,
		};

		for (mapped_item, item) in iter	{
			let some_mapped_item = Some(mapped_item);
			if some_mapped_item <= min_mapped { // <= instead of < to get rightmost
				min = Some(item);
				min_mapped = some_mapped_item;
			}
		}
		min
		*/
	}

	fn partial_max_by<F, B>(self, mut f: F) -> Option<Self::Item>
		where F: FnMut(&Self::Item) -> B,
		      B: AlmostOrd,
	{
		/*
		//let mut f_ord = |el: &Self::Item| OrderedVal::new_unchecked( f(el) );
		self.map(|it| (OrderedVal::new_checked( f(&it)), it) ) // (Option<OrdVal<B>>, T)
			.filter(|tup| (&tup.0).is_some())
			.max_by(|tup| tup.0) //  |(ord_val, _)| ord_val)
			.map(|(_, it)| it);
			//.max_by(f_ord).map(|m| m.0)
		unimplemented!()
		*/

		/*
		// Ownership problem. max_by demands ownership but that would lead to
		// 1. unnecessary recalculation
		// 2. lifetimes forbidding the approach below
		self.map(|it| (OrderedVal::new_checked(f(&it)), it))
			.filter(|tup| !tup.0.is_some())
			.max_by(|tup: &(_, _)| &tup.0) <-- max_by is greedy and needs ownership
			.map(|tup| tup.1)
		*/
		/*
		match self.max_by(|it| OrderedVal::new_checked(f(it))) {
			Some(Some(OrderedVal(b))) => Some(b),
			//Some(None) |None   => None,
			_ => None,
		}
		*/
		// Some > None, always
		self.max_by(|it| OrderedVal::new_checked(f(it)))

		//let b = 32.;
		//let test = OrderedVal::<&'a _>::new_unchecked(&b);
		//unimplemented!()
		/*
		let mut max = None;
		let mut max_mapped = None;
		for (mapped_item, item) in self.map(|it| (f(&it), it))
			.filter(|tup| !tup.0.is_outside_order() )
		{
			let some_mapped_item = Some(mapped_item);
			if some_mapped_item >= max_mapped { // >= instead of > to get rightmost
				max = Some(item);
				max_mapped = some_mapped_item;
			}
		}
		max
		*/
	}
}
