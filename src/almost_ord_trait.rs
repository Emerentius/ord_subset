/// Trait for types that form a total order when a few values are disallowed.
///
/// `is_outside_order()` must return `true` for these outliers and `false` for anything else.
///
/// `std::cmp::PartialOrd::partial_cmp(a,b)` must return `Some(_)` if a,b are both inside order and `None` if only one is outside order. Return value for two variables outside order is undefined.
pub trait AlmostOrd: PartialOrd<Self> + PartialEq<Self> {
	fn is_outside_order(&self) -> bool;

	/* would be nice sometimes, but providing this allows implementors to override it
	// which is always a logic error
	#[inline(always)]
	fn is_inside_order(&self) -> bool {
		!self.is_outside_order()
	}
	*/
}
/* Needs negative trait bounds or mutually exclusive traits
impl<T: Ord + !AlmostOrd> AlmostOrd for T {
	fn is_outside_order(&self) -> bool {
		false
	}
}
*/
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
