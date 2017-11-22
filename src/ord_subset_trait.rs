// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Trait for types that form a total order when a few values are disallowed.
///
/// `is_outside_order()` must return `true` for these outliers and `false` for anything else.
///
/// `std::cmp::PartialOrd::partial_cmp(a,b)` must return `Some(_)` if a,b are both inside order and `None` if only one is outside order. Return value for two variables outside order is undefined.
pub trait OrdSubset: PartialOrd<Self> + PartialEq<Self> {
	fn is_outside_order(&self) -> bool;
}

impl<'a, A> OrdSubset for &'a A where A: OrdSubset {
	#[inline(always)]
	fn is_outside_order(&self) -> bool {
		(*self).is_outside_order()
	}
}

impl<'a, A> OrdSubset for &'a mut A where A: OrdSubset {
	#[inline(always)]
	fn is_outside_order(&self) -> bool {
		(**self).is_outside_order()
	}
}

impl OrdSubset for f64 {
	#[inline(always)]
	fn is_outside_order(&self) -> bool {
		// only NaNs != itself
		*self != *self
	}
}

impl OrdSubset for f32 {
	#[inline(always)]
	fn is_outside_order(&self) -> bool {
		// only NaNs != itself
		*self != *self
	}
}

// Small helper used a lot in sorts
pub(crate) trait CmpUnwrap: OrdSubset {
	#[inline(always)]
	fn cmp_unwrap(&self, other: &Self) -> ::core::cmp::Ordering {
		self.partial_cmp(other).expect("Violated OrdSubset contract: a.partial_cmp(b) == None for a,b inside total order")
	}
}

impl<T: OrdSubset> CmpUnwrap for T {}
