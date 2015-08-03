use std::cmp::Ordering;
use std::fmt::Debug;
use almost_ord_trait::*;

// helper struct for AlmostOrd values. It's illegal to save values outside order in this.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct OrderedVal<T: AlmostOrd>(pub T);

#[allow(dead_code)]
impl<T: AlmostOrd> OrderedVal<T> {
	pub fn new(data: T)	-> OrderedVal<T>
		where T: Debug
	{
		if data.is_outside_order() { panic!("Tried to save unordered data into OrderedVal: {:?}", data) };
		OrderedVal(data)
	}

	pub fn new_checked(data: T)	-> Option<OrderedVal<T>>
	{
		match data.is_outside_order() {
			true  => None,
			false => Some(OrderedVal(data)),
		}
	}

	pub fn new_unchecked(data: T) -> OrderedVal<T> {
		OrderedVal(data)
	}
}

impl<T: AlmostOrd> Eq for OrderedVal<T> {}

impl<T: AlmostOrd> Ord for OrderedVal<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).expect("Violated AlmostOrd contract. OrderedVal internal wrapper contains non-ordered value")
	}
}
