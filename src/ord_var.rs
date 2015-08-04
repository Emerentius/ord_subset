use std::cmp::Ordering;
use std::fmt::Debug;
use almost_ord_trait::*;
use std::ops::Deref;

/// Wrapper for AlmostOrd types to signal that the contained variables have a total order. It's illegal to save values outside the total order in this.
///
/// # Panics
///
/// Panics when `a.partial_cmp(b)` returns `None` for two values `a`,`b` inside the total order (Violated AlmostOrd contract).
///
/// Also panics when an unsafely constructed OrdVar containing an invalid value is compared with another OrdVar and `.partial_cmp` returns `None`.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct OrdVar<T: AlmostOrd>(T);

impl<T: AlmostOrd> OrdVar<T> {

	/// Construct an ```OrdVar``` out of the argument.
	///
	/// # Panics
	///
	/// Panics if the argument is outside of the total order.
	pub fn new(data: T)	-> OrdVar<T>
		where T: Debug
	{
		if data.is_outside_order() { panic!("Attempted saving data outside of total order into OrdVar: {:?}", data) };
		OrdVar(data)
	}

	/// Constructs an ```Option<OrdVar>``` out of the argument. Returns None if the argument is outside the total order.
	pub fn new_checked(data: T)	-> Option<OrdVar<T>>
	{
		match data.is_outside_order() {
			true  => None,
			false => Some(OrdVar(data)),
		}
	}

	/// Constructs an ```OrdVar``` without validity check. Incorrectly constructed OrdVar's may panic during comparisons.
	pub unsafe fn new_unchecked(data: T) -> OrdVar<T> {
		OrdVar(data)
	}

	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<T: AlmostOrd> Eq for OrdVar<T> {}

impl<T: AlmostOrd> Ord for OrdVar<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).expect("OrdVar contains value outside total order")
	}
}

impl<T: AlmostOrd> Deref for OrdVar<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
