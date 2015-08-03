use almost_ord_trait::*;

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
